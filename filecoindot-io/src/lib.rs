// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[runtime_interface]
pub trait ForestProofVerify {
    fn verify_receipt(_proof: Vec<Vec<u8>>, _cid: Vec<u8>) -> Option<()> {
        use filecoindot_proofs::ProofVerify;
        None
    }

    fn verify_state(_proof: Vec<Vec<u8>>, _cid: Vec<u8>) -> Option<()> {
        use filecoindot_proofs::ProofVerify;
        None
    }
}
