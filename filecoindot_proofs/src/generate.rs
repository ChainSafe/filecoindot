// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Generation of proofs for AMT and HAMT tries

use crate::{Result, TrieError};
use ipld_amt::Amt;
use ipld_hamt::Hamt;

/// Generate an inclusion proof for the value in the given AMT.
pub fn generate_amt_proof<'db, V, BS>(amt: &Amt<'db, V, BS>, value: V) -> Result<Vec<Vec<u8>>> {
    // the stack of nodes representing the path in the trie
    let mut stack: Vec<u8> = vec![];

    // The trie nodes comprising the final proof
    let mut proof_nodes = Vec::new();

    Ok(proof_nodes)
}

/// Generate an inclusion proof for the value in the given HAMT.
pub fn generate_hamt_proof<'db, V, BS>(amt: &Hamt<'db, BS, V>, value: V) -> Result<Vec<Vec<u8>>> {
    // each node can have up to 32 children

    todo!()
}
