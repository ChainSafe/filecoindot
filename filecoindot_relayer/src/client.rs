// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! RPC client for requesting data from filecoin RPC

use crate::{
    api::{Api, ChainGetTipSetByHeight, ChainGetTipSetByHeightResult},
    cache::Cache,
    Env, Result,
};
use reqwest::Client as ReqwestClinet;

/// RPC Client of filecoindot relayers
pub struct Client {
    /// base url of rpc endpoint
    pub base: String,
    /// api cache
    pub cache: Cache,
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
            cache: Cache::new()?,
            inner: ReqwestClinet::new(),
        })
    }

    /// `Filecoin.ChainGetTipSetByHeight`
    ///
    /// Get tipset by block height
    ///
    /// ```
    /// # use filecoindot_relayer::Result;
    /// use filecoindot_relayer::{Client};
    ///
    /// # fn main() -> Result<()> {
    ///   assert_eq!(
    ///       tokio_test::block_on(
    ///           Client::new(None)?
    ///               .chain_get_tip_set_by_height(1199840),
    ///       )?,
    ///       filecoindot_relayer::testing::get_tip_set_by_height_1199840(),
    ///   );
    ///
    ///   # Ok(())
    /// }
    /// ```
    pub async fn chain_get_tip_set_by_height(
        &self,
        number: usize,
    ) -> Result<ChainGetTipSetByHeightResult> {
        Ok(Api::req(&ChainGetTipSetByHeight, self, vec![Some(number), None]).await?)
    }
}
