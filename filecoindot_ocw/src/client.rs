// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! RPC client for requesting data from filecoin RPC

use crate::{
    api::{Api, ChainGetTipSetByHeight, ChainGetTipSetByHeightResult},
    Env, Result,
};

/// RPC Client of filecoindot ocws
pub struct Client {
    /// base url of rpc endpoint
    pub base: String,
}

impl Client {
    /// New client with rpc endpoint
    ///
    /// If passing None, will try to get rpc endpoint
    /// from environment variables.
    pub fn new(rpc: Option<String>) -> Result<Self> {
        Ok(Self {
            base: rpc.unwrap_or(Env::rpc()?),
        })
    }

    // /// `Filecoin.ChainGetTipSetByHeight`
    // ///
    // /// Get tipset by block height
    // ///
    // /// ```
    // /// # use filecoindot_ocw::Result;
    // /// use filecoindot_ocw::{Client};
    // ///
    // /// # fn main() -> Result<()> {
    // ///   assert_eq!(
    // ///       tokio_test::block_on(
    // ///           Client::new(None)?
    // ///               .chain_get_tip_set_by_height(1199840),
    // ///       )?,
    // ///       filecoindot_ocw::testing::get_tip_set_by_height_1199840(),
    // ///   );
    // ///
    // ///   # Ok(())
    // /// }
    // /// ```
    pub fn chain_get_tip_set_by_height(number: u64) -> Result<ChainGetTipSetByHeightResult> {
        Ok(Api::req(&ChainGetTipSetByHeight, vec![Some(number), None])?)
    }
}

// #[test]
// fn test_api() {
//     Client::chain_get_tip_set_by_height(1199840);
// }
