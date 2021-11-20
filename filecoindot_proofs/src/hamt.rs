// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::Cid;
use ipld_hamt::Hash;
use std::marker::PhantomData;

use crate::errors::Error;
use crate::traits::{BlockStore, HAMTNode, HashAlgorithm, HashedBits};

/// This is a simplified implementation of HAMT based on:
/// http://lampwww.epfl.ch/papers/idealhashtrees.pdf
///
/// This implementation has only implemented the read related functions
/// as we only care about generating the path to the node
#[derive(Debug)]
pub struct Hamt<'a, BS, K: Eq, V, H: HashedBits, N: HAMTNode<K, V, H>, HashAlgo> {
    root: N,
    store: &'a BS,
    bit_width: u8,
    hash: PhantomData<HashAlgo>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _h: PhantomData<H>,
}

impl<'a, BS, K, V, H, N, HashAlgo> Hamt<'a, BS, K, V, H, N, HashAlgo>
where
    K: Eq + Hash,
    H: HashedBits,
    HashAlgo: HashAlgorithm<Output = H>,
    N: HAMTNode<K, V, H> + for<'de> serde::Deserialize<'de>,
    BS: BlockStore,
{
    /// Lazily instantiate a hamt from this root Cid with a specified bit width.
    pub fn new(root_cid: &Cid, store: &'a BS, bit_width: u8) -> Result<Self, Error> {
        let root: N = store.get(root_cid)?;
        Ok(Self {
            root,
            store,
            bit_width,
            hash: Default::default(),
            _k: Default::default(),
            _v: Default::default(),
            _h: Default::default(),
        })
    }

    /// Generates the path of all node bytes from the root to the node that contains the
    /// key. Returns Error::KeyNotFound if the key is not present in the tree.
    pub fn generate_proof(&self, k: &K) -> Result<Vec<Vec<u8>>, Error> {
        let mut path = Vec::new();
        if self.root.path_to_key(
            &mut HashAlgo::hash(k),
            k,
            &mut path,
            self.bit_width,
            self.store,
        )? {
            Ok(path)
        } else {
            Err(Error::NotFound)
        }
    }
}
