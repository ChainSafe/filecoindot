use crate::errors::Error;
use crate::node::{KeyValuePair, NodeInner};
use crate::traits::{BitMap, BlockStore, HashedBits, Node};
use cid::Cid;
use forest_db::{MemoryDB, Store};
use forest_encoding::from_slice;
use ipld_hamt::{Bitfield, HashBits, Pointer, Sha256};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;
use std::marker::PhantomData;

pub struct ForestAdaptedHashedBits<'a> {
    inner: HashBits<'a>,
}

impl<'a> HashedBits for ForestAdaptedHashedBits<'a> {
    type Value = u32;

    fn next(&mut self, n: u8) -> Result<Self::Value, Error> {
        self.inner.next(n as u32).map_err(|e| e.into())
    }
}

impl From<ipld_hamt::Error> for Error {
    fn from(_: ipld_hamt::Error) -> Self {
        Error::Other("TODO: ipld_hamt error".into())
    }
}

pub struct ForestAdaptedBitMap {
    inner: Bitfield,
}

impl BitMap for ForestAdaptedBitMap {
    type Index = u32;

    fn is_bit_set(&self, index: Self::Index) -> bool {
        self.inner.test_bit(index)
    }

    fn pop_count(&self, n: Self::Index) -> usize {
        let mask = Bitfield::zero().set_bits_le(n);
        mask.and(&self.inner).count_ones()
    }
}

pub struct ForestAdaptedBlockStorage {
    store: MemoryDB,
}

impl<'a, K, V, B: BitMap, N: Node<K, V, B>> BlockStore<K, V, B, N> for ForestAdaptedBlockStorage
where
    K: for<'de> serde::Deserialize<'de> + Eq,
    V: for<'de> serde::Deserialize<'de>,
{
    fn get<T>(&self, cid: &Cid) -> Result<Option<N>, Error> {
        match self.store.read(cid.to_bytes())? {
            Some(bytes) => {
                let (bitfield, pointers) = from_slice(&*bytes)?;
                let bitfield: Bitfield = bitfield;
                let pointers: Vec<Pointer<K, V, Sha256>> = pointers;
                todo!()
            }
            None => Ok(None),
        }
    }
}

impl From<forest_db::Error> for Error {
    fn from(_: forest_db::Error) -> Self {
        Error::Other("TODO: forest_db error".into())
    }
}
