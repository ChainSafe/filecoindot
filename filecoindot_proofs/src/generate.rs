// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Generation of proofs for AMT and HAMT tries

use crate::{Result, TrieError};

use ipld_hamt::Hamt;

pub mod amt {
    use crate::{Result, TrieError};
    use cid::Cid;
    use ipld_amt::{Amt, Node, Root};

    /// The type to keep track of a node's location in the path
    struct StackEntry {
        node_hash: Vec<u8>,

        /// Leaf references used when the proof is constructed
        leafs: Vec<LeafReference>,
        /// index into the final proof vector for this node
        output_index: Option<usize>,
    }

    /// A reference to a leaf node under a certain node
    enum LeafReference {
        ///
        Value(Cid),
        // TODO this could be a subtree
        Link(usize),
    }

    /// Generate an inclusion proof for the value in the given AMT.
    ///
    /// A node can either be a leaf node that contains the values or a link node that contains an array of other Cid or cached sub nodes
    pub fn generate_proof<'bs, V, BS>(node: &Amt<'bs, V, BS>, value: V) -> Result<Vec<Vec<u8>>> {
        // the stack of nodes representing the path in the trie where each entry is a child node of the preceding entry
        let mut stack: Vec<StackEntry> = vec![];

        // The trie nodes comprising the final proof
        let mut proof_nodes = Vec::new();

        // loop over the all nodes of the tree until we've found the leaf value and store

        Ok(proof_nodes)
    }
}

pub mod hamt {
    use crate::{Result, TrieError};
    use ipld_hamt::{Hamt, Node};

    /// Generate an inclusion proof for the value in the given HAMT.
    pub fn generate_proof<'db, V, BS>(hamt: &Hamt<'db, BS, V>, value: V) -> Result<Vec<Vec<u8>>> {
        // each node can have up to 32 children

        todo!()
    }
}
