// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use jsonrpc_core::types::error::{Error as RpcError, ErrorCode};

/// filecoindot rpc result
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid filecoin rpc endpoint")]
    InvalidEndpoint,
}

impl Into<RpcError> for Error {
    fn into(self) -> RpcError {
        match self {
            Error::InvalidEndpoint => RpcError {
                code: ErrorCode::InvalidRequest,
                message: self.to_string(),
                data: None,
            },
        }
    }
}
