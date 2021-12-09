// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

#![deny(warnings)]
use codec::{Decode, Encode};

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Error decoding from hex: {0}")]
    FromHex(#[from] hex::FromHexError),
    #[error("codec error: {0}")]
    CodecError(#[from] codec::Error),
}

/// encode a Vec of Vec of bytes into a String using hex encoding
pub fn hex_encode_proof(proof: Vec<Vec<u8>>) -> String {
    hex::encode(proof.encode())
}

/// decode a hex String into a Vec of Vec of bytes
pub fn decode_proof_from_hex(hex: &str) -> Result<Vec<Vec<u8>>, DecodeError> {
    let p = hex::decode(hex)?;
    let decoded = Decode::decode(&mut &*p)?;
    Ok(decoded)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_encode_proof_works() {
        let p = vec![
            vec![
                1, 113, 160, 228, 2, 32, 36, 124, 182, 126, 106, 187, 25, 199, 230, 181, 100, 214,
                154, 77, 62, 109, 17, 9, 120, 21, 205, 111, 102, 96, 38, 79, 186, 148, 178, 110,
                68, 137,
            ],
            vec![
                1, 113, 160, 228, 2, 32, 181, 170, 59, 78, 87, 6, 123, 107, 23, 248, 104, 224, 201,
                4, 132, 237, 73, 29, 249, 91, 139, 26, 156, 212, 179, 175, 127, 214, 118, 157, 251,
                48,
            ],
        ];

        let expected = "08980171a0e40220247cb67e6abb19c7e6b564d69a4d3e6d11097815cd6f6660264fba94b26e4489980171a0e40220b5aa3b4e57067b6b17f868e0c90484ed491df95b8b1a9cd4b3af7fd6769dfb30";
        let hex_string = hex_encode_proof(p);
        assert_eq!(hex_string, expected);
    }

    #[test]
    fn decode_hex_works() {
        let input = "08980171a0e40220247cb67e6abb19c7e6b564d69a4d3e6d11097815cd6f6660264fba94b26e4489980171a0e40220b5aa3b4e57067b6b17f868e0c90484ed491df95b8b1a9cd4b3af7fd6769dfb30";

        let expected = vec![
            vec![
                1, 113, 160, 228, 2, 32, 36, 124, 182, 126, 106, 187, 25, 199, 230, 181, 100, 214,
                154, 77, 62, 109, 17, 9, 120, 21, 205, 111, 102, 96, 38, 79, 186, 148, 178, 110,
                68, 137,
            ],
            vec![
                1, 113, 160, 228, 2, 32, 181, 170, 59, 78, 87, 6, 123, 107, 23, 248, 104, 224, 201,
                4, 132, 237, 73, 29, 249, 91, 139, 26, 156, 212, 179, 175, 127, 214, 118, 157, 251,
                48,
            ],
        ];
        let proof = decode_proof_from_hex(input).expect("must not error");
        assert_eq!(proof, expected);
    }
}
