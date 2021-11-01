use crate::errors::Error;
use crate::traits::{BlockStore, HashAlgorithm, HashedBits, Node};
use cid::Cid;
use cid::Code::Blake2b256;
use forest_encoding::{from_slice, to_vec};
use ipld_blockstore::BlockStore as ForestBlockStore;
use ipld_hamt::{
    Bitfield, Hash, HashAlgorithm as ForestHashAlgo, Node as ForestNode, Pointer as ForestPointer,
    Sha256,
};
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

impl From<ipld_blockstore::Error> for Error {
    fn from(_: ipld_blockstore::Error) -> Self {
        Error::Other("TODO: ipld_blockstore error".into())
    }
}

impl From<ipld_hamt::Error> for Error {
    fn from(_: ipld_hamt::Error) -> Self {
        Error::Other("TODO: ipld_hamt error".into())
    }
}

impl From<forest_db::Error> for Error {
    fn from(error: forest_db::Error) -> Self {
        let error_str = format!("forest_db error: {:?}", error);
        Error::Other(error_str)
    }
}

impl From<forest_encoding::error::Error> for Error {
    fn from(error: forest_encoding::error::Error) -> Self {
        let error_str = format!("forest_encoding error: {:?}", error);
        Error::Other(error_str)
    }
}

pub(crate) struct ForestAdaptedNode<K: Eq + Serialize, V: Serialize, Hash, HashOutput: HashedBits> {
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

impl<'a, K, V, H> Node<K, V, ForestAdaptedHashedBits>
    for ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>
where
    K: Eq + Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
{
    fn path_to_key<S: BlockStore<K, V, ForestAdaptedHashedBits, Self>>(
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
                if let Some(cached_node) = cache.get() {
                    path.push(cid.to_bytes());
                    let n = serialize_to_node(
                        Some(*cid),
                        &deserialize_node_slice(cached_node)?,
                    )?;
                    n.path_to_key(hash_bits, k, path, bit_width, s)
                } else {
                    let n = s.get(cid)?;
                    n.path_to_key(hash_bits, k, path, bit_width, s)
                }
            }
            ForestPointer::Dirty(n) => {
                let n = serialize_to_node(None, &deserialize_node_slice(n)?)?;
                n.path_to_key(hash_bits, k, path, bit_width, s)
            }
            ForestPointer::Values(key_values) => match key_values.iter().find(|kv| kv.key() == k) {
                Some(_) => {
                    path.push(self.cid()?.to_bytes());
                    Ok(true)
                }
                None => Ok(false),
            },
        }
    }

    fn cid(&self) -> Result<Cid, Error> {
        match self.cid {
            Some(cid) => Ok(cid),
            None => self.derive_cid(),
        }
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

impl<'a, K, H, V, FBS>
    BlockStore<K, V, ForestAdaptedHashedBits, ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>>
    for ForestAdaptedBlockStorage<FBS>
where
    K: Eq + Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
    FBS: ForestBlockStore,
{
    fn get(&self, cid: &Cid) -> Result<ForestAdaptedNode<K, V, H, ForestAdaptedHashedBits>, Error> {
        let n = self.store.read(cid.to_bytes())?.ok_or(Error::NotFound)?;
        serialize_to_node(Some(*cid), &n)
    }
}

fn serialize_to_node<
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

fn deserialize_node_slice<
    'a,
    K: Serialize + for<'de> serde::Deserialize<'de>,
    V: Serialize + for<'de> serde::Deserialize<'de>,
    H,
>(
    node: &ForestNode<K, V, H>,
) -> Result<Vec<u8>, Error> {
    Ok(to_vec(node)?)
}

// #[cfg(feature="forest")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hamt::Hamt;
    use ipld_blockstore::MemoryDB;
    use ipld_hamt::Hamt as ForestHamt;

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
        let hamt: Hamt<
            ForestAdaptedBlockStorage<MemoryDB>,
            usize,
            String,
            ForestAdaptedHashedBits,
            ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
            ForestAdaptedHashAlgo,
        > = Hamt::new(&cid, &store, 8).unwrap();
        let p = hamt.generate_proof(&100);
        assert_eq!(p.is_ok(), true);
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
}
