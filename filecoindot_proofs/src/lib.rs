
#![cfg_attr(not(feature = "std"), no_std)]

//! Supports proof generation and verification for filecoin's:
//!
//!   _AMT_: Array Mapped Trie. A data structure used by Filecoin to store
//! messages in a block.
//!   _HAMT_: Hash Array Mapped Trie. Data structure used by
//! Filecoin to store the key-value map of the chain state.


// mod types;