// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! RPC client for requesting data from filecoin RPC

use crate::{
    api::{Req, CHAIN_GET_BLOCK},
    types::Block,
    Env, Result,
};
use reqwest::Client as ReqwestClinet;

/// RPC Client of filecoindot relayers
pub struct Client {
    /// base url of rpc endpoint
    pub base: String,
    /// inner rpc client
    pub inner: ReqwestClinet,
}

impl Client {
    /// New client with rpc endpoint
    ///
    /// If passing None, will try to get rpc endpoint
    /// from environment variables.
    pub fn new(rpc: Option<String>) -> Result<Self> {
        Ok(Self {
            base: rpc.unwrap_or(Env::rpc()?),
            inner: ReqwestClinet::new(),
        })
    }

    /// "Filecoin.ChainGetBlock"
    ///
    /// Get `Block` by block number
    ///
    // /// ```
    // /// use relayer::Client;
    // ///
    // /// let client = Client::new(None).unwrap();
    // /// println!("{:?}", tokio_test::block_on(client.block(42)));
    // /// ```
    pub async fn block(&self, number: usize) -> Result<Block> {
        CHAIN_GET_BLOCK.req(self, &[&number.to_string()]).await
    }
}
