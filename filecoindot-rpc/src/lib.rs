// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use jsonrpc_derive::rpc;
use result::{Error, Result};
use sp_core::offchain::StorageKind;

/// filecoin rpc config
pub const FILECOIN_RPC: &[u8] = b"FILECOIN_RPC";

mod result;

/// filecointdot rpc api
#[rpc]
pub trait FilecoindotApi {
    /// set filecoin rpc endpoint for filecoindot
    #[rpc(name = "filecoindot_setRpcEndpoint")]
    fn set_rpc_endpoint(&self, url: String) -> Result<()>;
}

/// filecoindot rpc handler
pub struct Filecoindot;

impl FilecoindotApi for Filecoindot {
    fn set_rpc_endpoint(&self, url: String) -> Result<()> {
        if url.starts_with("http") {
            sp_io::offchain::local_storage_set(
                StorageKind::PERSISTENT,
                FILECOIN_RPC,
                url.as_bytes(),
            );
            Ok(())
        } else {
            Err(Error::InvalidEndpoint)
        }
    }
}
