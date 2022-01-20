// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(clippy::enum_variant_names)]
use derive_more::Display;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum Error {
    #[display(fmt = "failed to parse bytes as str")]
    FormatBytesFailed,
    #[display(fmt = "get offchain worker storage failed")]
    GetStorageFailed,
    #[display(fmt = "haven't set filecoin rpc yet")]
    FilecoinRpcNotSet,
    #[display(fmt = "blocks and cids not matched in tipset")]
    InvalidTipSet,
    #[display(fmt = "http request failed")]
    HttpError,
    #[display(fmt = "signed tx error")]
    OffchainSignedTxError,
    #[display(fmt = "no tx result yet")]
    NoTxResult,
    #[display(fmt = "no relayer found")]
    NoRelayerFound,
}

pub type Result<T> = core::result::Result<T, Error>;
