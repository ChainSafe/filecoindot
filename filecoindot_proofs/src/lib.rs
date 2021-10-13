// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

// #![cfg_attr(not(feature = "std"), no_std)]

//! Supports proof generation and verification for filecoin's:
//!
//!   _AMT_: Array Mapped Trie. A data structure used by Filecoin to store
//! messages in a block.
//!   _HAMT_: Hash Array Mapped Trie. Data structure used by
//! Filecoin to store the key-value map of the chain state.

mod generate;
mod verify;

/// Trie Errors.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TrieError {
    /// The proof is missing trie nodes that are required for verification.
    IncompleteProof,
}

/// Trie result type.
pub type Result<T> = core::result::Result<T, TrieError>;
