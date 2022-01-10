// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::errors::Error;
use crate::traits::{BlockStore, GetCid, HAMTNode, HashAlgorithm, HashedBits};
use cid::Cid;
use cid::Code::Blake2b256;
use forest_encoding::de::{Deserialize, Deserializer};
use forest_encoding::{from_slice, to_vec};
use ipld_blockstore::BlockStore as ForestBlockStore;
use ipld_hamt::{
    Bitfield, Hash, HashAlgorithm as ForestHashAlgo, Node as ForestNode, Pointer as ForestPointer,
    Pointer, Sha256,
};
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::marker::PhantomData;

pub struct ForestAdaptedHashAlgo;

impl HashAlgorithm for ForestAdaptedHashAlgo {
    type Output = ForestAdaptedHashedBits;

    fn hash<X: ?Sized + Hash>(key: &X) -> Self::Output {
        ForestAdaptedHashedBits {
            b: Sha256::hash(key),
            consumed: 0,
        }
    }
}

#[inline]
const fn mkmask(n: u32) -> u32 {
    ((1u64 << n) - 1) as u32
}

pub struct ForestAdaptedHashedBits {
    b: [u8; 32],
    pub consumed: u32,
}

impl ForestAdaptedHashedBits {
    fn next_bits(&mut self, i: u32) -> u32 {
        let curbi = self.consumed / 8;
        let leftb = 8 - (self.consumed % 8);

        let curb = self.b[curbi as usize] as u32;
        match i.cmp(&leftb) {
            Ordering::Equal => {
                // bits to consume is equal to the bits remaining in the currently indexed byte
                let out = mkmask(i) & curb;
                self.consumed += i;
                out
            }
            Ordering::Less => {
                // Consuming less than the remaining bits in the current byte
                let a = curb & mkmask(leftb);
                let b = a & !mkmask(leftb - i);
                let c = b >> (leftb - i);
                self.consumed += i;
                c
            }
            Ordering::Greater => {
                // Consumes remaining bits and remaining bits from a recursive call
                let mut out = (mkmask(leftb) & curb) as u64;
                out <<= i - leftb;
                self.consumed += leftb;
                out += self.next_bits(i - leftb) as u64;
                out as u32
            }
        }
    }
}

impl HashedBits for ForestAdaptedHashedBits {
    type Value = u32;

    /// Returns next `i` bits of the hash and returns the value as an integer and returns
    /// Error when maximum depth is reached
    fn next(&mut self, i: u8) -> Result<Self::Value, Error> {
        let i = i as u32;
        if i > 8 {
            return Err(Error::InvalidHashBitLen);
        }
        if (self.consumed + i) as usize > self.b.len() * 8 {
            return Err(Error::MaxDepth);
        }
        Ok(self.next_bits(i))
    }
}

pub struct ForestAdaptedNode<K: Eq + Serialize, V: Serialize, Hash, HashOutput: HashedBits> {
    cid: Option<Cid>,
    // we keep the original data for cid derivation
    bitfield: Bitfield,
    raw_pointers: Vec<ForestPointer<K, V, Hash>>,
    _h: PhantomData<HashOutput>,
}

impl<K, V, Hash, HashOutput> Serialize for ForestAdaptedNode<K, V, Hash, HashOutput>
where
    K: Eq + Serialize,
    V: Serialize,
    HashOutput: HashedBits,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.bitfield, &self.raw_pointers).serialize(serializer)
    }
}

impl<'de, K, V, Hash, HashOutput> Deserialize<'de> for ForestAdaptedNode<K, V, Hash, HashOutput>
where
    HashOutput: HashedBits,
    K: Eq + Serialize + for<'a> serde::Deserialize<'a>,
    V: Serialize + for<'a> serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let (bitfield, pointers) = Deserialize::deserialize(deserializer)?;
        Ok(ForestAdaptedNode::new(None, bitfield, pointers))
    }
}

impl<K: Eq + Serialize, V: Serialize, H, HashOutput: HashedBits>
    ForestAdaptedNode<K, V, H, HashOutput>
{
    pub fn new(
        cid: Option<Cid>,
        bitfield: Bitfield,
        raw_pointers: Vec<ForestPointer<K, V, H>>,
    ) -> Self {
        ForestAdaptedNode {
            cid,
            bitfield,
            raw_pointers,
            _h: Default::default(),
        }
    }

    fn derive_cid(&self) -> Result<Cid, Error> {
        let bytes = to_vec(self)?;
        Ok(cid::new_from_cbor(&bytes, Blake2b256))
    }

    fn index_for_bit_pos(&self, bp: u32) -> usize {
        let mask = Bitfield::zero().set_bits_le(bp);
        assert_eq!(mask.count_ones(), bp as usize);
        mask.and(&self.bitfield).count_ones()
    }

    fn get_child(&self, i: usize) -> &ForestPointer<K, V, H> {
        &self.raw_pointers[i]
    }
}

impl<K, V, H> GetCid for ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>
where
    K: Eq + Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
{
    fn cid(&self) -> Result<Cid, Error> {
        match self.cid {
            Some(cid) => Ok(cid),
            None => self.derive_cid(),
        }
    }
}

impl<'a, K, V, H> HAMTNode<K, V, ForestAdaptedHashedBits>
    for ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>
where
    K: Eq + Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
{
    fn path_to_key<S: BlockStore>(
        &self,
        hash_bits: &mut ForestAdaptedHashedBits,
        k: &K,
        path: &mut Vec<Vec<u8>>,
        bit_width: u8,
        s: &S,
    ) -> Result<bool, Error> {
        let idx = hash_bits.next(bit_width)?;

        if !self.bitfield.test_bit(idx) {
            return Ok(false);
        }

        let cindex = self.index_for_bit_pos(idx);
        let child = self.get_child(cindex);
        match child {
            ForestPointer::Link { cid, cache, .. } => {
                path.push(to_vec(self)?);
                let n: Self;
                if let Some(cached_node) = cache.get() {
                    n = deserialize_to_node(Some(*cid), &serialize_to_slice(cached_node)?)?;
                    n.path_to_key(hash_bits, k, path, bit_width, s)
                } else {
                    n = s.get(cid)?;
                    n.path_to_key(hash_bits, k, path, bit_width, s)
                }
            }
            ForestPointer::Dirty(n) => {
                path.push(to_vec(self)?);
                let n: Self = deserialize_to_node(None, &serialize_to_slice(n)?)?;
                n.path_to_key(hash_bits, k, path, bit_width, s)
            }
            ForestPointer::Values(key_values) => match key_values.iter().find(|kv| kv.key() == k) {
                Some(_) => {
                    path.push(to_vec(self)?);
                    Ok(true)
                }
                None => Ok(false),
            },
        }
    }

    fn get_by_cid<S: BlockStore>(&self, cid: &Cid, store: &S) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        for pointer in &self.raw_pointers {
            match pointer {
                Pointer::Values(_) => {
                    continue;
                }
                Pointer::Link {
                    cid: link_cid,
                    cache,
                    ..
                } => {
                    if cid != link_cid {
                        continue;
                    }
                    return if let Some(cached_node) = cache.get() {
                        let node =
                            deserialize_to_node(Some(*cid), &serialize_to_slice(cached_node)?)?;
                        Ok(Some(node))
                    } else {
                        let n = store.get(link_cid)?;
                        Ok(Some(n))
                    };
                }
                // TODO: check again if dirty needs to be checked
                Pointer::Dirty(_) => {
                    continue;
                }
            }
        }
        Ok(None)
    }
}

pub struct ForestAdaptedBlockStorage<FBS: ForestBlockStore> {
    store: FBS,
}

impl<FBS: ForestBlockStore> ForestAdaptedBlockStorage<FBS> {
    pub fn new(store: FBS) -> Self {
        ForestAdaptedBlockStorage { store }
    }
}

impl<FBS> BlockStore for ForestAdaptedBlockStorage<FBS>
where
    FBS: ForestBlockStore,
{
    fn get<T: DeserializeOwned>(&self, cid: &Cid) -> Result<T, Error> {
        let n = self.store.read(cid.to_bytes())?.ok_or(Error::NotFound)?;
        Ok(from_slice(&n)?)
    }
}

pub fn deserialize_to_node<
    'a,
    K: Eq + Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
    H,
>(
    cid: Option<Cid>,
    bytes: &[u8],
) -> Result<ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>, Error> {
    let (bitfield, pointers): (Bitfield, Vec<ForestPointer<K, V, H>>) = from_slice(bytes)?;
    Ok(ForestAdaptedNode::new(cid, bitfield, pointers))
}

pub fn serialize_to_slice<
    'a,
    K: Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
    H,
>(
    node: &ForestNode<K, V, H>,
) -> Result<Vec<u8>, Error> {
    Ok(to_vec(node)?)
}

pub type HAMTNodeType =
    ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, ForestAdaptedHashedBits>;

pub type MessageNodeType =
    ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, ForestAdaptedHashedBits>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hamt::Hamt;
    use crate::{ProofVerify, Verify};
    use ipld_blockstore::MemoryDB;
    use ipld_hamt::Hamt as ForestHamt;

    type HamtType<'a> = Hamt<
        'a,
        ForestAdaptedBlockStorage<MemoryDB>,
        usize,
        String,
        ForestAdaptedHashedBits,
        HAMTNodeType,
        ForestAdaptedHashAlgo,
    >;

    #[test]
    fn test_basic_proof_generation() {
        let bs = MemoryDB::default();
        let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

        let max = 1000;
        for i in 1..max {
            fhamt.set(i, i.to_string()).unwrap();
        }

        let cid = fhamt.flush().unwrap();
        let store = ForestAdaptedBlockStorage::new(bs);
        let hamt: Hamt<
            ForestAdaptedBlockStorage<MemoryDB>,
            usize,
            String,
            ForestAdaptedHashedBits,
            ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
            ForestAdaptedHashAlgo,
        > = Hamt::new(&cid, &store, 8).unwrap();
        for i in 1..max {
            let p = hamt.generate_proof(&i);
            assert_eq!(p.is_ok(), true);
        }
    }

    #[test]
    fn test_deeper_tree() {
        let bs = MemoryDB::default();
        let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

        let max = 10000;
        for i in 1..max {
            fhamt.set(i, i.to_string()).unwrap();
        }

        let cid = fhamt.flush().unwrap();
        let store = ForestAdaptedBlockStorage::new(bs);
        let hamt: HamtType = Hamt::new(&cid, &store, 8).unwrap();
        let p = hamt.generate_proof(&(max - 1));
        assert_eq!(p.is_ok(), true);
        let v = p.unwrap();
        assert_eq!(v.len(), 2);
        // assert_eq!(v[0], cid.to_bytes());
    }

    #[test]
    fn test_not_found() {
        let bs = MemoryDB::default();
        let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

        let max = 1000;
        for i in 1..max {
            fhamt.set(i, i.to_string()).unwrap();
        }

        let cid = fhamt.flush().unwrap();
        let store = ForestAdaptedBlockStorage::new(bs);
        let hamt: Hamt<
            ForestAdaptedBlockStorage<MemoryDB>,
            usize,
            String,
            ForestAdaptedHashedBits,
            ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
            ForestAdaptedHashAlgo,
        > = Hamt::new(&cid, &store, 8).unwrap();

        let p = hamt.generate_proof(&(max + 1));
        assert_eq!(p.is_err(), true);
    }

    #[test]
    fn test_verify_works() {
        let bs = MemoryDB::default();
        let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

        let max = 10000;
        for i in 1..max {
            fhamt.set(i, i.to_string()).unwrap();
        }

        let cid = fhamt.flush().unwrap();
        let store = ForestAdaptedBlockStorage::new(bs);
        let hamt: Hamt<
            ForestAdaptedBlockStorage<MemoryDB>,
            usize,
            String,
            ForestAdaptedHashedBits,
            ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
            ForestAdaptedHashAlgo,
        > = Hamt::new(&cid, &store, 8).unwrap();

        let mut p = hamt.generate_proof(&(max / 2)).unwrap();
        p.reverse();

        let raw_node = p.get(0).unwrap();
        let node: HAMTNodeType = deserialize_to_node(None, raw_node).unwrap();
        let r = ProofVerify::verify_proof::<HAMTNodeType>(p, node.cid().unwrap().to_bytes());
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn test_verify_not_ok() {
        let bs = MemoryDB::default();
        let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

        let max = 10000;
        for i in 1..max {
            fhamt.set(i, i.to_string()).unwrap();
        }

        let cid = fhamt.flush().unwrap();
        let store = ForestAdaptedBlockStorage::new(bs);
        let hamt: Hamt<
            ForestAdaptedBlockStorage<MemoryDB>,
            usize,
            String,
            ForestAdaptedHashedBits,
            ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
            ForestAdaptedHashAlgo,
        > = Hamt::new(&cid, &store, 8).unwrap();

        let mut p = hamt.generate_proof(&(max / 2)).unwrap();
        p.reverse();
        let target_cid = cid::new_from_cbor(&[1, 2, 3], Blake2b256);
        let r = ProofVerify::verify_proof::<HAMTNodeType>(p, target_cid.to_bytes());
        assert_eq!(r.is_ok(), false);
    }
}
