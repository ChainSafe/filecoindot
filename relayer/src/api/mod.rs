// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

mod get_tip_set_by_height;

pub use self::get_tip_set_by_height::{ChainGetTipSetByHeight, ChainGetTipSetByHeightResult};
use crate::{Client, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Wrapper for jsonrpc result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resp<T> {
    /// reponse id
    pub id: usize,
    /// JsonRPC version
    pub jsonrpc: String,
    /// JsonRPC result
    pub result: T,
}

/// Request JSON body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Req<'r, T> {
    /// reponse id
    pub id: usize,
    /// JsonRPC method
    pub method: &'r str,
    /// JsonRPC version
    pub jsonrpc: &'r str,
    /// JsonRPC result
    pub params: T,
}

/// Abstract filecoin api requests
#[async_trait]
pub trait Api {
    const METHOD: &'static str;
    type Params: Serialize + Send + Sync;
    type Result: DeserializeOwned;

    /// Request method with params
    async fn req(&self, client: &Client, params: Self::Params) -> Result<Self::Result> {
        // let context = client
        //     .inner
        //     .post(&client.base)
        //     .json(&Req {
        //         id: 0,
        //         method: Self::METHOD,
        //         jsonrpc: "2.0",
        //         params,
        //     })
        //     .send()
        //     .await?;
        //
        // panic!("{}", context.text().await?);
        // Err(Error::NoRPCEndpoint)
        Ok(client
            .inner
            .post(&client.base)
            .json(&Req {
                id: 0,
                method: Self::METHOD,
                jsonrpc: "2.0",
                params,
            })
            .send()
            .await?
            .json::<Resp<Self::Result>>()
            .await?
            .result)
    }
}
