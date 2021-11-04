// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("could not find FILECOINT_RPC in environment variables")]
    NoRPCEndpoint,
    #[error("Only supports http endpoints for now")]
    NotHttpEndpoint,
    #[error("rpc request failed")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Serialize storage key failed")]
    BuildStorageKeyFailed(#[from] bincode::Error),
    #[error("serde_json error")]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
