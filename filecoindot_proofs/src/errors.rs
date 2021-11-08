// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

/// Database error
#[derive(Debug)]
pub enum Error {
    InvalidHashBitLen,
    MaxDepth,
    NotFound,
    VerificationFailed,
    MaxHeightExceeded,
    CidNotFound(String),
    Other(String),
}
