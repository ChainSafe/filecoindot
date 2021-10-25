// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

/// Database error
#[derive(Debug)]
pub enum Error {
    InvalidBulkLen,
    Unopened,
    InvalidHashBitLen,
    MaxDepth,
    CidNotFound(String),
    Other(String),
}

impl From<forest_encoding::error::Error> for Error {
    fn from(_: forest_encoding::error::Error) -> Self {
        todo!()
    }
}
