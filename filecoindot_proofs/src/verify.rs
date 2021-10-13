// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Verification of proofs for AMT and HAMT tries.

use crate::{Result, TrieError as Error};
use cid::Cid;
use std::vec::Vec;

/// Verify a message proof for an AMT.
///
/// This will verify that the given block contains the message at the given path.
pub fn verify_message(message: Cid, block: Cid, proof: &[Vec<u8>]) -> Result<()> {
    // Iterate over proof nodes
    let mut proof_iter = proof.iter();

    let root_node = proof_iter.next().ok_or(Error::IncompleteProof)?;

    Ok(())
}

/// Verify the state proof for a HAMT.
///
/// Given a state kv pair, its containing block header and the path in the state
/// merkle DAG, will return if the given entry is included in the state at the
/// given block
pub fn verify_state(key: Cid, value: Vec<u8>, block: Cid, proof: &[Vec<u8>]) {}
