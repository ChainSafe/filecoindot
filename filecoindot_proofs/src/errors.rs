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
    IPLDAmt(ipld_amt::Error),
    IPLDHamt(ipld_hamt::Error),
    ForestDB(forest_db::Error),
    CborEncoding(serde_cbor::Error),
    BlockStore(ipld_blockstore::Error),
    Other(String),
}
