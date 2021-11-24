// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::errors::Error;
use cid::Cid;
use ipld_hamt::Hash;
use num_traits::Num;
use serde::de::DeserializeOwned;

pub trait HashAlgorithm {
    type Output: HashedBits;
    fn hash<X: ?Sized + Hash>(key: &X) -> Self::Output;
}

pub trait HashedBits {
    type Value: Num + Copy;
    fn next(&mut self, n: u8) -> Result<Self::Value, Error>;
}

pub trait GetCid {
    fn cid(&self) -> Result<Cid, Error>;
}

pub trait HAMTNode<K, V, H>: GetCid
where
    K: Eq,
    H: HashedBits,
{
    fn path_to_key<S: BlockStore>(
        &self,
        hash_bits: &mut H,
        k: &K,
        path: &mut Vec<Vec<u8>>,
        bit_width: u8,
        store: &S,
    ) -> Result<bool, Error>;

    fn get_by_cid<S: BlockStore>(&self, cid: &Cid, store: &S) -> Result<Option<Self>, Error>
    where
        Self: Sized;
}

pub trait AMTNode: GetCid {
    fn path_to_key<S: BlockStore>(
        &self,
        store: &S,
        bit_width: usize,
        height: usize,
        i: usize,
        path: &mut Vec<Vec<u8>>,
    ) -> Result<bool, Error>;

    fn get_by_cid<S: BlockStore>(
        &self,
        cid: &Cid,
        store: &S,
        bit_width: usize,
    ) -> Result<Option<Self>, Error>
    where
        Self: Sized;
}

/// Wrapper for database to handle inserting and retrieving ipld data with Cids
pub trait BlockStore {
    /// Get typed object from block store by Cid.
    fn get<T: DeserializeOwned>(&self, cid: &Cid) -> Result<T, Error>;
}

/// The proof verification trait
pub trait Verify {
    fn verify_proof<N>(proof: Vec<Vec<u8>>, node_cid: Vec<u8>) -> Result<(), Error>
    where
        N: GetCid + for<'de> serde::Deserialize<'de>;
}
