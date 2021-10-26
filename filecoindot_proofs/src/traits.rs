// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::errors::Error;
use cid::Cid;
use num_traits::Num;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait HashAlgorithm {
    type Output: HashedBits;
    fn hash<X: ?Sized>(key: &X) -> Self::Output;
}

pub trait HashedBits {
    type Value: Num;
    fn get(&self) -> Self::Value;
    fn next(&mut self) -> Result<Self::Value, Error>;
}

pub trait Node<K: Eq, V> {
    type GetHashBits: HashedBits;

    /// Whether it contains the key
    fn contains_key(&self, key: &K, hashed_bits: &Self::GetHashBits) -> bool;

    /// Checks if the node is of Index NodeType
    fn is_index(&self) -> bool;

    /// Returns a clone of the cid.
    /// TODO: consider returning Option<&Cid> in the future
    fn cid(&self) -> Cid;

    /// Get the cid to the idx of the node
    /// TODO: consider returning Option<&Cid> in the future
    fn get_link_cid(
        &self,
        idx: <<Self as Node<K, V>>::GetHashBits as HashedBits>::Value,
    ) -> Option<Cid>;
}

/// Wrapper for database to handle inserting and retrieving ipld data with Cids
pub trait BlockStore<K: Eq, V, N: Node<K, V>> {
    /// Get typed object from block store by Cid.
    fn get<T>(&self, cid: &Cid) -> Result<Option<N>, Error>;
}

pub(crate) trait BitMap {
    type Index: Num;
    /// The max number of bits this BitMap can hold
    fn size(&self) -> usize;
    /// Clear the bit at the specified index, required index < size()
    fn clear_bit(&mut self, index: Self::Index) -> Result<(), Error>;
    /// Checks if the bit at index is set
    fn is_bit_set(&self, index: Self::Index) -> bool;
    /// Performs the and operation between two BitMaps
    fn and(&self, rhs: Self) -> Self;
    /// Get the count of 1 bit in the start of n indexes
    fn count_ones(&self, n: Self::Index) -> usize;
}
