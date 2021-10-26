// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::borrow::Borrow;
use std::marker::PhantomData;

use cid::{Cid, Code::Blake2b256};
use forest_hash_utils::BytesKey;
use serde::{de::DeserializeOwned, Serialize, Serializer};

use crate::errors::Error;
use crate::hash::Hash;
use crate::traits::{BlockStore, HashAlgorithm, HashedBits, Node};

/// This is a simplified implementation of HAMT based on:
/// http://lampwww.epfl.ch/papers/idealhashtrees.pdf
///
/// This implementation has only implemented the read related functions
/// as we only care about generating the path to the node
#[derive(Debug)]
pub struct Hamt<'a, BS, K: Eq, V, N: Node<K, V>, HashAlgo> {
    root: N,
    store: &'a BS,
    bit_width: u8,
    hash: PhantomData<HashAlgo>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<'a, BS, K, V, HashOutput, N, HashAlgo> Hamt<'a, BS, K, V, N, HashAlgo>
where
    K: Eq,
    HashOutput: HashedBits,
    BS: BlockStore<K, V, N>,
    HashAlgo: HashAlgorithm<Output = HashOutput>,
    N: Node<K, V, GetHashBits = HashOutput>,
{
    /// Lazily instantiate a hamt from this root Cid with a specified bit width.
    pub fn new(
        root_cid: &Cid,
        store: &'a BS,
        bit_width: u8,
    ) -> Result<Self, Error> {
        match store.get::<N>(root_cid)? {
            Some(root) => Ok(Self {
                root,
                store,
                bit_width,
                hash: Default::default(),
                _k: Default::default(),
                _v: Default::default(),
            }),
            None => Err(Error::CidNotFound(root_cid.to_string())),
        }
    }

    pub fn generate_proof(&self, k: &K) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut path = Vec::new();

        if !self.path_to_key(&self.root, &mut HashAlgo::hash(k), k, &mut path)? {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn path_to_key(
        &self,
        node: &N,
        hash_bits: &mut HashOutput,
        k: &K,
        path: &mut Vec<Vec<u8>>,
    ) -> Result<bool, Error> {
        if !node.contains_key(k, hash_bits) {
            return Ok(false);
        }

        // we have reached a KeyValue node
        if !node.is_index() {
            path.push(node.cid().to_bytes());
            Ok(true)
        } else {
            let idx = hash_bits.next()?;
            // unwrap should be safe as node has passed contains_key
            let cid = node.get_link_cid(idx).unwrap();
            match self.store.get::<N>(&cid)? {
                Some(node) => {
                    path.push(node.cid().to_bytes());
                    self.path_to_key(&node, hash_bits, k, path)
                }
                None => Err(Error::CidNotFound(cid.to_string())),
            }
        }
    }
}
