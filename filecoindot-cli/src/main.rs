// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

#![deny(warnings)]
use anyhow::Result;
use cid::Cid;
use filecoindot_cli::{decode_proof_from_hex, DecodeError};
use filecoindot_proofs::generic_verify;
use std::convert::TryFrom;
use thiserror::Error;
use type_cli::CLI;

#[derive(CLI)]
enum Filecoindot {
    Verify {
        #[named]
        proof: String,
        #[named]
        cid: String,
    },
}

#[derive(Error, Debug)]
enum CliError {
    #[error("CID error: {0}")]
    Cid(#[from] filecoindot_proofs::cid::Error),
    #[error("Verification Error: {0}")]
    Verification(#[from] filecoindot_proofs::Error),
    #[error("decode error: {0}")]
    Decode(#[from] DecodeError),
}

fn main() -> Result<(), CliError> {
    match Filecoindot::process() {
        Filecoindot::Verify { proof, cid } => {
            let proof = decode_proof_from_hex(&proof)?;
            let cid = Cid::try_from(&*cid)?;
            generic_verify(proof, &cid)?;
            println!("verification success");
            Ok(())
        }
    }
}
