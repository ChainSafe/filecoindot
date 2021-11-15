// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::{Cid};
use forest_encoding::de::Deserializer;
use serde::{Deserialize};
use crate::errors::Error;
use crate::traits::{AMTNode, BlockStore};

pub fn nodes_for_height(bit_width: usize, height: usize) -> usize {
    let height_log_two = bit_width * height;
    if height_log_two >= 64 {
        return std::usize::MAX;
    }
    1 << height_log_two
}

const MAX_HEIGHT: usize = 8;
const MAX_INDEX: usize = (u64::MAX - 1) as usize;

#[derive(Debug)]
pub struct Amt<'db, BS: BlockStore, N: AMTNode> {
    node: N,
    block_store: Option<&'db BS>,
    bit_width: usize,
    height: usize,
    count: usize,
}

impl <'db, 'de, BS: BlockStore, N: AMTNode + Deserialize<'de>> Deserialize<'de> for Amt<'db, BS, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let (bit_width, height, count, node): (_, _, _, N) =
            Deserialize::deserialize(deserializer)?;
        Ok(Self {
            bit_width,
            height,
            count,
            node,
            block_store: None
        })
    }
}

impl<'db, BS, N> Amt<'db, BS, N>
where
    BS: BlockStore,
    N: AMTNode + for<'de> Deserialize<'de>
{

    /// Constructs an AMT with a blockstore and a Cid of the root of the AMT
    pub fn load(cid: &Cid, block_store: &'db BS) -> Result<Self, Error> {
        // Load root bytes from database
        let mut root = block_store
            .get::<Self>(cid)?;

        // Sanity check, this should never be possible.
        if root.height > MAX_HEIGHT {
            return Err(Error::MaxHeightExceeded);
        }

        root.block_store = Some(block_store);
        Ok(root)
    }

    /// Get value at index of AMT
    pub fn generate_proof(&self, i: usize) -> Result<Vec<Vec<u8>>, Error> {
        if i > MAX_INDEX {
            return Err(Error::NotFound);
        }

        if i >= nodes_for_height(self.bit_width, self.height + 1) {
            return Err(Error::NotFound);
        }

        let mut path = Vec::new();
        if self.node
            .path_to_key(*self.block_store.as_ref().unwrap(), self.bit_width, self.height, i, &mut path)? {
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
        if root_cid != self.node.cid()? {
            return Err(Error::VerificationFailed);
        }

        self.traverse_and_match(&proof, proof.len() - 1, &self.node, node_cid)?;
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
            .get_by_cid(&next_cid, *self.block_store.as_ref().unwrap(), self.bit_width)
            .map_err(|_| Error::VerificationFailed)?
            // node with next_cid not found in the current node, fail directly
            .ok_or(Error::VerificationFailed)?;
        self.traverse_and_match(proof, index - 1, &next_node, target_cid)
    }
}
