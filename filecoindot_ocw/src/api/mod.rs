// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

mod get_tip_set_by_height;

pub use self::get_tip_set_by_height::{ChainGetTipSetByHeight, ChainGetTipSetByHeightResult};
use crate::{Env, Error, Result};
use frame_support::sp_runtime::offchain::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Wrapper for jsonrpc result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resp<T> {
    /// reponse id
    pub id: u8,
    /// JsonRPC version
    pub jsonrpc: String,
    /// JsonRPC result
    pub result: T,
}

/// Request JSON body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Req<T> {
    /// reponse id
    pub id: u8,
    /// JsonRPC method
    pub method: String,
    /// JsonRPC version
    pub jsonrpc: String,
    /// JsonRPC result
    pub params: T,
}

/// Abstract filecoin api requests
pub trait Api: Sized {
    const METHOD: &'static str;
    type Params: Serialize + Send + Sync;
    type Result: Serialize + DeserializeOwned + core::fmt::Debug;

    /// Storage key in bytes
    fn storage_key(params: &Self::Params) -> Result<Vec<u8>> {
        let mut key = bincode::serialize(Self::METHOD)?;
        key.append(&mut bincode::serialize(params)?);
        Ok(key)
    }

    /// Request method with params
    fn req(&self, params: Self::Params) -> Result<Self::Result> {
        let base = Env::rpc()?;
        let req = Request::post(
            &base,
            vec![serde_json::to_vec(&Req {
                id: 0,
                method: Self::METHOD.to_string(),
                jsonrpc: "2.0".to_string(),
                params,
            })
            .unwrap()],
        )
        .add_header("Content-Type", "application/json");

        let pending = req.send().unwrap();
        let response = pending.wait().unwrap();
        let resp_bytes = response.body().collect::<Vec<_>>();
        println!("resp bytes: {}", resp_bytes.len());
        println!("resp text: {}", String::from_utf8_lossy(&resp_bytes));
        panic!(
            "de: {:?}",
            serde_json::from_slice::<Resp<Self::Result>>(&resp_bytes)?.result
        );
        Err(Error::DirectoryNotFound)
        // }
    }
}
