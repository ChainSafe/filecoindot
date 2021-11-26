#![deny(warnings)]
use anyhow::Result;
use cid::Cid;
use filecoindot_cli::decode_proof_from_hex;
use filecoindot_proofs::generic_verify;
use std::convert::TryFrom;
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

fn main() -> anyhow::Result<()> {
    match Filecoindot::process() {
        Filecoindot::Verify { proof, cid } => {
            let proof = decode_proof_from_hex(&proof)?;
            let cid = Cid::try_from(&*cid)?;
            match generic_verify(proof, &cid) {
                Ok(_r) => {
                    println!("verification success");
                }
                Err(e) => {
                    println!("verification failed: {}", e);
                }
            }
        }
    }
    Ok(())
}
