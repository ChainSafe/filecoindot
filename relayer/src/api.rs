// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

use crate::{Client, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Abstract filecoin api requests
#[async_trait]
pub trait Req {
    /// Request method with params
    async fn req<T: DeserializeOwned>(&self, client: &Client, params: &[&str]) -> Result<T>;
}

#[async_trait]
impl Req for &'static str {
    /// request this path with response
    async fn req<T: DeserializeOwned>(&self, client: &Client, params: &[&str]) -> Result<T> {
        Ok(client
            .inner
            .post(&client.base)
            .form(&[
                ("id", "0"),
                ("jsonrpc", "2.0"),
                ("method", self),
                ("params", &format!("{:?}", params)),
            ])
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}

/// Method `Filecoin.ChainGetBlock`
pub const CHAIN_GET_BLOCK: &str = "Filecoin.ChainGetBlock";
