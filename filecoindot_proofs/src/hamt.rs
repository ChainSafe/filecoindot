// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::marker::PhantomData;
use cid::Cid;

use crate::errors::Error;
use crate::node::Pointer;
use crate::traits::{BitMap, BlockStore, HashAlgorithm, HashedBits, Node};

/// This is a simplified implementation of HAMT based on:
/// http://lampwww.epfl.ch/papers/idealhashtrees.pdf
///
/// This implementation has only implemented the read related functions
/// as we only care about generating the path to the node
#[derive(Debug)]
pub struct Hamt<'a, BS, K: Eq, V, B: BitMap, N: Node<K, V, B>, HashAlgo> {
    root: N,
    store: &'a BS,
    bit_width: u8,
    hash: PhantomData<HashAlgo>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _b: PhantomData<B>,
}

impl<'a, BS, K, B, V, HashOutput, N, HashAlgo> Hamt<'a, BS, K, V, B, N, HashAlgo>
where
    K: Eq,
    B: BitMap<Index = HashOutput::Value>,
    HashOutput: HashedBits,
    BS: BlockStore<K, V, B, N>,
    HashAlgo: HashAlgorithm<Output = HashOutput>,
    N: Node<K, V, B>,
{
    /// Lazily instantiate a hamt from this root Cid with a specified bit width.
    pub fn new(root_cid: &Cid, store: &'a BS, bit_width: u8) -> Result<Self, Error> {
        match store.get::<N>(root_cid)? {
            Some(root) => Ok(Self {
                root,
                store,
                bit_width,
                hash: Default::default(),
                _k: Default::default(),
                _v: Default::default(),
                _b: Default::default(),
            }),
            None => Err(Error::CidNotFound(root_cid.to_string())),
        }
    }

    pub fn generate_proof(&self, k: &K) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut path = Vec::new();
        if self.path_to_key(&self.root, &mut HashAlgo::hash(k), k, &mut path)? {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    /// This is an implementation of the search outlined in the paper: see section 3.1
    fn path_to_key(
        &self,
        node: &N,
        hash_bits: &mut HashOutput,
        k: &K,
        path: &mut Vec<Vec<u8>>,
    ) -> Result<bool, Error> {
        let idx = hash_bits.next(self.bit_width)?;
        let bitmap = node.bitmap();

        if !bitmap.is_bit_set(idx) {
            return Ok(false);
        }

        match node.get_pointer(bitmap.pop_count(idx)).unwrap() {
            Pointer::KeyValue(key_values) => match key_values.iter().find(|kv| kv.key() == k) {
                Some(_) => {
                    path.push(node.cid().to_bytes());
                    Ok(true)
                }
                None => Ok(false),
            },
            Pointer::Link(cid) => match self.store.get::<N>(&cid)? {
                Some(n) => self.path_to_key(&n, hash_bits, k, path),
                None => Ok(false),
            },
        }
    }
}
