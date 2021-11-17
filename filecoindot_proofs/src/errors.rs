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
    IPLDAmtError(ipld_amt::Error),
    IPLDHamtError(ipld_hamt::Error),
    ForestDBError(forest_db::Error),
    CborEncodingError(serde_cbor::Error),
    BlockStoreError(ipld_blockstore::Error),
    Other(String),
}
