// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "haven't set filecoin rpc yet")]
    FilecoinRpcNotSet,
}

pub type Result<T> = core::result::Result<T, Error>;
