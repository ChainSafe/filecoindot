// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

pub use self::chain_head::ChainHead;
use frame_support::{
    log,
    sp_runtime::offchain::{
        http::{Error, Request},
        Timestamp,
    },
    sp_std::{vec, vec::Vec},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod chain_head;

/// Wrapper for jsonrpc result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resp<T> {
    /// JsonRPC result
    pub result: T,
}

/// Request JSON body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Req<T> {
    /// reponse id
    pub id: u8,
    /// JsonRPC method
    pub method: &'static str,
    /// JsonRPC version
    pub jsonrpc: &'static str,
    /// JsonRPC result
    pub params: T,
}

/// Abstract filecoin api requests
pub trait Api: Sized {
    const METHOD: &'static str;
    type Params: Serialize + Send + Sync + Clone;
    type Result: Serialize + DeserializeOwned + core::fmt::Debug;

    fn iter_req(
        &self,
        endpoints: &[&str],
        params: Self::Params,
        deadline: Timestamp,
    ) -> Result<Self::Result, Error> {
        let mut result = Err(Error::IoError);
        for endpoint in endpoints {
            result = self.req(endpoint, params.clone(), deadline);
            if result.is_ok() {
                return result;
            }
        }

        result
    }

    /// Request method with params
    fn req(
        &self,
        endpoint: &str,
        params: Self::Params,
        deadline: Timestamp,
    ) -> Result<Self::Result, Error> {
        let body = serde_json::to_vec(&Req {
            id: 0,
            method: Self::METHOD,
            jsonrpc: "2.0",
            params,
        })
        .map_err(|_| Error::IoError)?;

        // build request
        let req = Request::post(endpoint, vec![body])
            .add_header("Content-Type", "application/json")
            .deadline(deadline);

        // get response
        let resp = req
            .send()
            .map_err(|e| {
                log::error!("send request failed {:?}", e);
                Error::IoError
            })?
            .wait()
            .map_err(|e| {
                log::error!("wait request faild {:?}", e);
                Error::IoError
            })?
            .body()
            .collect::<Vec<_>>();

        // deserialize response
        Ok(serde_json::from_slice::<Resp<Self::Result>>(&resp)
            .map_err(|e| {
                log::error!("result {:?}", resp);
                log::error!("parse result failed {:?}", e);
                Error::IoError
            })?
            .result)
    }
}
