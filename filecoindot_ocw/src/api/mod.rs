// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin APIs

mod get_tip_set_by_height;

pub use self::get_tip_set_by_height::{ChainGetTipSetByHeight, ChainGetTipSetByHeightResult};
use crate::{Error, Result};
use async_trait::async_trait;
use codec::{Decode, Encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sp_runtime::offchain::{http::Request, storage::StorageValueRef};

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
#[async_trait]
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
        let key = <Self>::storage_key(&params)?;
        let val = StorageValueRef::local(&key);
        if let Ok(Some(res)) = val.get::<Self::Result>() {
            Ok(res)
        } else {
            let req = Request::post(
                "http://httpbin/post",
                vec![Req {
                    id: 0,
                    method: Self::METHOD.to_string(),
                    jsonrpc: "2.0".to_string(),
                    params,
                }
                .encode()],
            );
            let res = req
                .send()
                .unwrap()
                .wait()
                .unwrap()
                .body()
                .collect::<Vec<_>>();

            // panic!("{}", String::decode(&mut res.as_ref()).unwrap());
            Self::Result::decode(&mut res.as_ref()).unwrap();
            // .json(&Req {
            //     id: 0,
            //     method: Self::METHOD,
            //     jsonrpc: "2.0",
            //     params: &params,
            // })
            // .send()
            // .await?
            // .json::<Resp<Self::Result>>()
            // .await?
            // .result;
            // client
            //     .cache
            //     .put::<Self>(&params, &bincode::serialize(&res)?)?;
            // res
            Err(Error::DirectoryNotFound)
        }
    }
}
