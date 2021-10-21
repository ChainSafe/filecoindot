// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

mod get_tip_set_by_height;

pub use self::get_tip_set_by_height::{ChainGetTipSetByHeight, ChainGetTipSetByHeightResult};
use crate::{Error, Result};
use codec::{Decode, Encode};
use frame_support::sp_runtime::offchain::{http::Request, storage::StorageValueRef};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Wrapper for jsonrpc result
#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Resp<T: Encode + Decode> {
    /// reponse id
    pub id: u8,
    /// JsonRPC version
    pub jsonrpc: String,
    /// JsonRPC result
    pub result: T,
}

/// Request JSON body
#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Req<T: Encode + Decode> {
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
    type Params: Encode + Serialize + Send + Sync + Decode;
    type Result: Serialize + Decode + DeserializeOwned;

    /// Storage key in bytes
    fn storage_key(params: &Self::Params) -> Result<Vec<u8>> {
        let mut key = bincode::serialize(Self::METHOD)?;
        key.append(&mut bincode::serialize(params)?);
        Ok(key)
    }

    /// Request method with params
    fn req(&self, params: Self::Params) -> Result<Self::Result> {
        // let key = <Self>::storage_key(&params)?;
        // let val = StorageValueRef::local(&key);
        // if let Ok(Some(res)) = val.get::<Self::Result>() {
        //     println!("{:?}", res);
        //     Ok(res)
        // } else {
        let req = Request::get(
            "http://httpbin.org/get",
            // vec![Req {
            //     id: 0,
            //     method: Self::METHOD.to_string(),
            //     jsonrpc: "2.0".to_string(),
            //     params,
            // }
            // .encode()],
        );
        let pending = req.send().unwrap();

        let response = pending.wait().unwrap();
        // .body()
        // .collect::<Vec<_>>();

        // let decode = String::decode(&mut res.as_ref()).unwrap();

        panic!(
            "{:?}",
            String::from_utf8_lossy(&mut response.body().collect::<Vec<u8>>().as_ref())
        );
        Err(Error::DirectoryNotFound)
        // }
    }
}
