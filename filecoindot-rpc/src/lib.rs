// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use filecoindot_proofs::{ForestAmtAdaptedNode, HAMTNodeType, ProofVerify, Verify};
use jsonrpc_derive::rpc;
use parking_lot::RwLock;
use result::{Error, Result};
use sp_core::{offchain::OffchainStorage, Encode};
use std::sync::Arc;
use url::Url;

/// filecoin rpc config
pub const FILECOIN_RPC: &[u8] = b"FILECOIN_RPC";

mod result;

/// filecointdot rpc api
#[rpc]
pub trait FilecoindotApi {
    /// set filecoin rpc endpoint for filecoindot
    #[rpc(name = "filecoindot_setRpcEndpoint")]
    fn set_rpc_endpoint(&self, urls: Vec<String>) -> Result<()>;

    // verify receipt
    #[rpc(name = "filecoindot_verifyReceipt")]
    fn verify_receipt(&self, proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Result<bool>;

    // verify state
    #[rpc(name = "filecoindot_verifyState")]
    fn verify_state(&self, proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Result<bool>;
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
    fn set_rpc_endpoint(&self, urls: Vec<String>) -> Result<()> {
        if urls.is_empty()
            || urls
                .iter()
                .any(|url| !url.starts_with("http") || Url::parse(url).is_err())
        {
            return Err(Error::InvalidEndpoint);
        }

        self.storage.write().set(
            sp_offchain::STORAGE_PREFIX,
            FILECOIN_RPC,
            &urls
                .iter()
                .map(|url| url.as_bytes().to_vec())
                .collect::<Vec<Vec<u8>>>()
                .encode(),
        );
        Ok(())
    }

    // verify receipt
    fn verify_receipt(&self, proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Result<bool> {
        Ok(ProofVerify::verify_proof::<ForestAmtAdaptedNode<String>>(proof, cid).is_ok())
    }

    // verify state
    fn verify_state(&self, proof: Vec<Vec<u8>>, cid: Vec<u8>) -> Result<bool> {
        Ok(ProofVerify::verify_proof::<HAMTNodeType>(proof, cid).is_ok())
    }
}
