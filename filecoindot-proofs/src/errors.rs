// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

/// Database error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Hashbit length")]
    InvalidHashBitLen,
    #[error("MaxDepth error")]
    MaxDepth,
    #[error("Not found")]
    NotFound,
    #[error("Proof verification failed")]
    VerificationFailed,
    #[error("Max height exceeded")]
    MaxHeightExceeded,
    #[error("Cid not found `{0}`")]
    CidNotFound(String),
    #[error("IPLD AMT error `{0}`")]
    IPLDAmt(#[from] ipld_amt::Error),
    #[error("IPLD HAMT error `{0}`")]
    IPLDHamt(#[from] ipld_hamt::Error),
    #[error("ForestDB error `{0}`")]
    ForestDB(#[from] forest_db::Error),
    #[error("CborEncoding error `{0}`")]
    CborEncoding(#[from] serde_cbor::Error),
    #[error("IPLD blockstore error `{0}`")]
    BlockStore(#[from] ipld_blockstore::Error),
    #[error("Generic error `{0}`")]
    Other(String),
}
