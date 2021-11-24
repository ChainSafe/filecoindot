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

    /// Given the CID of a message, its containing block and the path in
    /// the blocks messages merkle DAG.
    ///
    /// return true if the message is included in the given block and if
    /// that block has been voted as accepted.
    #[rpc(name = "filecoindot_verifyMessage")]
    fn verify_message(
        &self,
        message_cid: String,
        block_header: String,
        path: String,
    ) -> Result<bool>;

    /// API. Given the CID of a message, its containing block and the path in the
    /// blocks messages merkle DAG.
    ///
    /// return if the message is included in the given block and if that block
    /// has been voted as accepted.
    #[rpc(name = "filecoindot_verifyReceipt")]
    fn verify_receipt(
        &self,
        message_cid: String,
        block_header: String,
        path: String,
    ) -> Result<bool>;

    /// API. Given a state kv pair, its containing block header and the path in the
    /// state merkle DAG.
    ///
    /// return if the given entry is included in the state at the given block and
    /// if that block has been voted as accepted.
    #[rpc(name = "filecoindot_verifyState")]
    fn verify_state(&self, kv_pair: String, block_header: String, path: String) -> Result<bool>;
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

    fn verify_message(
        &self,
        _message_cid: String,
        _block_header: String,
        _path: String,
    ) -> Result<bool> {
        Ok(false)
    }

    fn verify_receipt(
        &self,
        _message_cid: String,
        _block_header: String,
        _path: String,
    ) -> Result<bool> {
        Ok(false)
    }

    fn verify_state(&self, _kv_pair: String, _block_header: String, _path: String) -> Result<bool> {
        Ok(false)
    }
}
