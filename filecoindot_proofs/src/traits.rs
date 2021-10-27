// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::errors::Error;
use crate::node::Pointer;
use cid::Cid;
use num_traits::Num;
use std::cell::RefMut;

pub trait HashAlgorithm {
    type Output: HashedBits;
    fn hash<X: ?Sized>(key: &X) -> Self::Output;
}

pub trait HashedBits {
    type Value: Num + Copy;
    fn next(&mut self, n: u8) -> Result<Self::Value, Error>;
}

pub trait Node<K, V, B>
where
    K: Eq,
    B: BitMap,
{
    fn bitmap(&self) -> RefMut<B>;

    fn get_pointer(&self, idx: usize) -> Option<&Pointer<K, V>>;

    /// Returns a clone of the cid.
    /// TODO: consider returning Option<&Cid> in the future
    fn cid(&self) -> Cid;
}

/// Wrapper for database to handle inserting and retrieving ipld data with Cids
pub trait BlockStore<K: Eq, V, B: BitMap, N: Node<K, V, B>> {
    /// Get typed object from block store by Cid.
    fn get<T>(&self, cid: &Cid) -> Result<Option<N>, Error>;
}

pub trait BitMap {
    type Index: Num + Copy;
    /// Checks if the bit at index is set
    fn is_bit_set(&self, index: Self::Index) -> bool;
    /// Get the count of 1 bit in the start of n indexes
    fn pop_count(&self, n: Self::Index) -> usize;
}
