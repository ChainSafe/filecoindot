// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Module for handling environments

use crate::{Error, Result};
use std::env::var;

pub const FILECOIN_RPC_ENV: &str = "FILECOIN_RPC";

/// A set of reserved environemts for relayer client
pub struct Env;

impl Env {
    /// Get environment variable `FILECOIN_RPC`
    pub fn rpc() -> Result<String> {
        let r = var(FILECOIN_RPC_ENV).map_err(|_| Error::NoRPCEndpoint)?;
        if !r.starts_with("http") {
            Err(Error::NotHttpEndpoint)
        } else {
            Ok(r)
        }
    }
}
