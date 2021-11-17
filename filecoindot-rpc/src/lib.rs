// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use jsonrpc_derive::rpc;
use parking_lot::RwLock;
use result::{Error, Result};
use sp_core::{offchain::OffchainStorage, Encode};
use std::sync::Arc;

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
pub struct Filecoindot<T: OffchainStorage> {
    storage: Arc<RwLock<T>>,
}

impl<T> Filecoindot<T>
where
    T: OffchainStorage,
{
    /// new filecoindot api
    pub fn new(storage: Arc<RwLock<T>>) -> Self {
        Self { storage }
    }
}

impl<T> FilecoindotApi for Filecoindot<T>
where
    T: OffchainStorage + 'static,
{
    fn set_rpc_endpoint(&self, url: String) -> Result<()> {
        if url.starts_with("http") {
            self.storage
                .write()
                .set(sp_offchain::STORAGE_PREFIX, FILECOIN_RPC, &url.encode());
            Ok(())
        } else {
            Err(Error::InvalidEndpoint)
        }
    }
}
