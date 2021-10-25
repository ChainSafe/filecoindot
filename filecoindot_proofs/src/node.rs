// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;
use cid::Cid;
use serde::de::DeserializeOwned;
use crate::bitmap::BitMap;

/// Node in Hamt tree which contains bitfield of set indexes and pointers to nodes
#[derive(Debug)]
enum NodeType<K, V, B: BitMap, N: Node<K, V>> {
    KeyValue{ key: K, val: V },
    Index {
        bitmap: B,
        /// Link nodes pointing to other nodes
        links: Vec<Box<N>>
    }
}

pub trait Node<K, V>: DeserializeOwned {
    /// Whether it contains the key
    fn contains_key(&self, key: &K) -> bool;

    /// Checks if the node is of Index NodeType
    fn is_index(&self) -> bool;

    /// Returns a clone of the cid.
    /// TODO: consider returning Option<&Cid> in the future
    fn cid(&self) -> Cid;

    /// Get the cid to the idx of the node
    /// TODO: consider returning Option<&Cid> in the future
    fn get_link_cid(&self, idx: u32) -> Option<Cid>;
}

