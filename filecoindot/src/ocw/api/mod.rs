// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

pub use self::chain_head::ChainHead;
use frame_support::{
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
    /// reponse id
    pub id: u8,
    /// JsonRPC version
    #[serde(skip_deserializing)]
    pub jsonrpc: Vec<u8>,
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
    type Params: Serialize + Send + Sync;
    type Result: Serialize + DeserializeOwned + core::fmt::Debug;

    /// Request method with params
    fn req(
        &self,
        base: &str,
        params: Self::Params,
        deadline: Timestamp,
    ) -> Result<Self::Result, Error> {
        // set env via storage
        let req = Request::post(
            base,
            vec![serde_json::to_vec(&Req {
                id: 0,
                method: Self::METHOD,
                jsonrpc: "2.0",
                params,
            })
            .map_err(|_| Error::IoError)?],
        )
        .add_header("Content-Type", "application/json")
        .deadline(deadline);

        Ok(serde_json::from_slice::<Resp<Self::Result>>(
            &req.send()
                .map_err(|_| Error::IoError)?
                .wait()
                .map_err(|_| Error::IoError)?
                .body()
                .collect::<Vec<_>>(),
        )
        .map_err(|_| Error::IoError)?
        .result)
    }
}
