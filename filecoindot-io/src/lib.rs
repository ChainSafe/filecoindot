// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[runtime_interface]
pub trait ForestProofVerify {
    fn verify_receipt(proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Option<()> {
        use filecoindot_proofs::{ForestAmtAdaptedNode, ProofVerify, Verify};
        ProofVerify::verify_proof::<ForestAmtAdaptedNode<String>>(proof, cid).ok()
    }

    fn verify_state(proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Option<()> {
        use filecoindot_proofs::{HAMTNodeType, ProofVerify, Verify};
        ProofVerify::verify_proof::<HAMTNodeType>(proof, cid).ok()
    }

    fn verify_message(proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Option<()> {
        use filecoindot_proofs::{MessageNodeType, ProofVerify, Verify};
        ProofVerify::verify_proof::<MessageNodeType>(proof, cid).ok()
    }
}

#[runtime_interface]
pub trait Benchmarking {
    fn hamt_proof_generation() -> (Vec<Vec<u8>>, Vec<u8>) {
        filecoindot_proofs::benchmarking::hamt_proof_generation()
    }

    fn amt_proof_generation(n: u64) -> (Vec<Vec<u8>>, Vec<u8>) {
        filecoindot_proofs::benchmarking::amt_proof_generation(n as usize)
    }
}
