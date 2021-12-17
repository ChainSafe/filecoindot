// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use jsonrpc_core::types::error::{Error as RpcError, ErrorCode};

/// filecoindot rpc result
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid filecoin rpc endpoint")]
    InvalidEndpoint,
    #[error("hex error: {0}")]
    FromHexError(#[from] hex::FromHexError),
    #[error("codec error: {0}")]
    CodecError(#[from] codec::Error),
    #[error("cid error: {0}")]
    CidError(#[from] cid::Error),
}

impl From<Error> for RpcError {
    fn from(e: Error) -> RpcError {
        match e {
            _ => RpcError {
                code: ErrorCode::InvalidRequest,
                message: e.to_string(),
                data: None,
            },
        }
    }
}
