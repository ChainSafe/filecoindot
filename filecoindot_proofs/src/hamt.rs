// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::borrow::Borrow;
use std::marker::PhantomData;

use cid::{Cid, Code::Blake2b256};
use forest_hash_utils::BytesKey;
use serde::{de::DeserializeOwned, Serialize, Serializer};

use crate::blockstore::BlockStore;
use crate::errors::Error;
use crate::hash::Hash;
use crate::hash_algorithm::{HashAlgorithm, Sha256};
use crate::hash_bits::HashBits;
use crate::node::Node;
use crate::KeyValuePair;

/// This is a simplified implementation of HAMT based on:
/// http://lampwww.epfl.ch/papers/idealhashtrees.pdf
///
/// This implementation has only implemented the read related functions
/// as we only care about generating the path to the node
#[derive(Debug)]
pub struct Hamt<'a, BS, K, V, N: Node<K, V>, H = Sha256> {
    root: N,
    store: &'a BS,
    bit_width: u32,
    hash: PhantomData<H>,
    _k: PhantomData<K>,
    _v: PhantomData<V>
}

impl<'a, BS, K, V, N, H> Hamt<'a, BS, K, V, N, H>
where
    K: Hash + Eq + PartialOrd + Serialize + DeserializeOwned,
    BS: BlockStore,
    H: HashAlgorithm,
    N: Node<K, V>
{
    /// Lazily instantiate a hamt from this root Cid with a specified bit width.
    pub fn load_with_bit_width(
        root_cid: &Cid,
        store: &'a BS,
        bit_width: u32,
    ) -> Result<Self, Error> {
        match store.get(root_cid)? {
            Some(root) => Ok(Self {
                root,
                store,
                bit_width,
                hash: Default::default(),
                _k: Default::default(),
                _v: Default::default()
            }),
            None => Err(Error::CidNotFound(root_cid.to_string())),
        }
    }

    pub fn generate_proof(&self, k: &K) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut path = Vec::new();

        if !self.path_to_key(&self.root, &mut HashBits::new(&H::hash(k)), k, &mut path)? {
            Ok(None)
        } else {
            Ok(Some(path))
        }
    }

    fn path_to_key(
        &self,
        node: &N,
        hashed_key: &mut HashBits,
        k: &K,
        path: &mut Vec<Vec<u8>>,
    ) -> Result<bool, Error> {
        if !node.contains_key(k) {
            return Ok(false);
        }

        // we have reached a KeyValue node
        if !node.is_index() {
            path.push(node.cid().to_bytes());
            Ok(true)
        } else {
            let idx = hashed_key.next(self.bit_width)?;
            // unwrap should be safe as node has passed contains_key
            let cid= node.get_link_cid(idx).unwrap();
            match self.store.get::<N>(&cid)? {
                Some(node) => {
                    path.push(node.cid().to_bytes());
                    self.path_to_key(&node, hashed_key, k, path)
                }
                None => Err(Error::CidNotFound(cid.to_string())),
            }
        }

    }
}
