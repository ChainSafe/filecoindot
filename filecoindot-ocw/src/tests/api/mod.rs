// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

mod get_tip_set_by_height;

pub use self::get_tip_set_by_height::{ChainGetTipSetByHeight, ChainGetTipSetByHeightResult};
use frame_support::{
    sp_runtime::offchain::http::{Error, Request},
    sp_std::{vec, vec::Vec},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Wrapper for jsonrpc result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resp<T> {
    /// reponse id
    pub id: u8,
    /// JsonRPC version
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
    pub method: Vec<u8>,
    /// JsonRPC version
    pub jsonrpc: Vec<u8>,
    /// JsonRPC result
    pub params: T,
}

/// Abstract filecoin api requests
pub trait Api: Sized {
    const METHOD: &'static str;
    type Params: Serialize + Send + Sync;
    type Result: Serialize + DeserializeOwned + core::fmt::Debug;

    /// Storage key in bytes
    fn storage_key(params: &Self::Params) -> Result<Vec<u8>, Error> {
        let mut key = bincode::serialize(Self::METHOD).map_err(|_| Error::IoError)?;
        key.append(&mut bincode::serialize(params).map_err(|_| Error::IoError)?);
        Ok(key)
    }

    /// Request method with params
    fn req(&self, base: &str, params: Self::Params) -> Result<Self::Result, Error> {
        // set env via storage
        let req = Request::post(
            &base,
            vec![serde_json::to_vec(&Req {
                id: 0,
                method: Self::METHOD.as_bytes().to_vec(),
                jsonrpc: "2.0".as_bytes().to_vec(),
                params,
            })
            .map_err(|_| Error::IoError)?],
        )
        .add_header("Content-Type", "application/json");

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
