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

    /// Generates a full path from the root to the node that contains the
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

    /// Verify the proof and the the trie actually matches. Each cid in the proof
    /// is connected to its neighbours. The proof should match exactly in path from
    /// the root to the node.
    /// Note that proof[proof.len-1] == root_cid_bytes. This function does not assume
    /// the head of the proof to be equal to node_cid, as long as it's in the proof.
    pub fn verify_proof(&self, proof: Vec<Vec<u8>>, node_cid: &Cid) -> Result<(), Error> {
        if proof.is_empty() {
            return Err(Error::VerificationFailed);
        }

        let root_cid =
            Cid::read_bytes(&*proof[proof.len() - 1]).map_err(|_| Error::VerificationFailed)?;
        if root_cid != self.root.cid()? {
            return Err(Error::VerificationFailed);
        }

        self.traverse_and_match(&proof, proof.len() - 1, &self.root, node_cid)?;
        Ok(())
    }

    fn traverse_and_match(
        &self,
        proof: &[Vec<u8>],
        index: usize,
        current_node: &N,
        target_cid: &Cid,
    ) -> Result<(), Error> {
        let current_node_cid = current_node.cid()?;
        if current_node_cid == *target_cid {
            return Ok(());
        }

        // We have not found the target_cid in the proof, search the next nodes

        // The index is 0, we have reached the end of the proof, cannot proceed
        // any further, return error.
        if index == 0 {
            return Err(Error::VerificationFailed);
        }

        // now we search the previous index as we traverse deeper in to the trie
        let next_cid =
            Cid::read_bytes(&*proof[index - 1]).map_err(|_| Error::VerificationFailed)?;
        let next_node = current_node
            .get_by_cid(&next_cid, self.store)
            .map_err(|_| Error::VerificationFailed)?
            // node with next_cid not found in the current node, fail directly
            .ok_or(Error::VerificationFailed)?;
        self.traverse_and_match(proof, index - 1, &next_node, target_cid)
    }
}
