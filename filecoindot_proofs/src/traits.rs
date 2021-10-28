// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::errors::Error;
use cid::Cid;
use ipld_hamt::Hash;
use num_traits::Num;

pub trait HashAlgorithm {
    type Output: HashedBits;
    fn hash<X: ?Sized + Hash>(key: &X) -> Self::Output;
}

pub trait HashedBits {
    type Value: Num + Copy;
    fn next(&mut self, n: u8) -> Result<Self::Value, Error>;
}

pub trait Node<K, V, H>
where
    K: Eq,
    H: HashedBits,
{
    fn path_to_key<S: BlockStore<K, V, H, Self>>(
        &self,
        hash_bits: &mut H,
        k: &K,
        path: &mut Vec<Vec<u8>>,
        bit_width: u8,
        store: &S,
    ) -> Result<bool, Error>
    where
        Self: Sized;

    fn cid(&self) -> Result<Cid, Error>;
}

/// Wrapper for database to handle inserting and retrieving ipld data with Cids
pub trait BlockStore<K, V, H, N>
where
    K: Eq,
    H: HashedBits,
    N: Node<K, V, H>,
{
    /// Get typed object from block store by Cid.
    fn get(&self, cid: &Cid) -> Result<Option<N>, Error>;
}
