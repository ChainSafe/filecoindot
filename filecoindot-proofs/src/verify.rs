// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::errors::Error;
use crate::traits::{GetCid, Verify};
use crate::HAMTNodeType;
use cid::Cid;
use serde_cbor::de::from_slice;
use std::convert::TryFrom;

pub struct ProofVerify;

impl ProofVerify {
    fn traverse_and_match<N>(proof: &[Vec<u8>], index: usize, target_cid: &Cid) -> Result<(), Error>
    where
        N: GetCid + for<'de> serde::Deserialize<'de>,
    {
        let current_node: N = from_slice(&*proof[index]).map_err(|_| Error::VerificationFailed)?;
        if current_node.cid()? == *target_cid {
            return Ok(());
        }

        // We have not found the target_cid in the proof, search the next nodes.
        // The index is 0, we have reached the end of the proof, cannot proceed
        // any further, return error.
        if index == 0 {
            return Err(Error::VerificationFailed);
        }

        // now we search the previous index as we traverse deeper in to the trie
        Self::traverse_and_match::<N>(proof, index - 1, target_cid)
    }
}

impl Verify for ProofVerify {
    /// Verify the proof and the the trie actually matches. Each cid in the proof
    /// is connected to its neighbours. The proof should match exactly in path from
    /// the root to the node.
    /// Note that proof[proof.len-1] == root_cid_bytes. This function does not assume
    /// the head of the proof to be equal to node_cid, as long as it's in the proof.
    fn verify_proof<N>(proof: Vec<Vec<u8>>, node_cid: Vec<u8>) -> Result<(), Error>
    where
        N: GetCid + for<'de> serde::Deserialize<'de>,
    {
        let node_cid = Cid::try_from(node_cid).map_err(|_| Error::VerificationFailed)?;
        if proof.is_empty() {
            return Err(Error::VerificationFailed);
        }
        Self::traverse_and_match::<N>(&proof, proof.len() - 1, &node_cid)
    }
}

/// Verify a proof against a Cid.
///
/// Note: this is using HAMTNodeType
pub fn generic_verify(proof: Vec<Vec<u8>>, cid: &Cid) -> Result<(), Error> {
    ProofVerify::verify_proof::<HAMTNodeType>(proof, cid.to_bytes())
}
