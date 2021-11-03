// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin api types
#![allow(missing_docs)]
use super::de;
use codec::{Decode, Encode};
use frame_support::sp_std::vec::Vec;
use serde::{Deserialize, Serialize};

/// Response of the [`ChainHead`](https://docs.filecoin.io/reference/lotus-api/#chainhead) RPC call
#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode, Eq, PartialEq)]
pub struct GetChainHead {
    pub jsonrpc: String,
    pub result: TipSet,
    pub id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode, Eq, PartialEq)]
pub struct TipSet {
    #[serde(rename = "Cids")]
    pub cids: Vec<Cid>,
    #[serde(rename = "Blocks")]
    pub blocks: Vec<Block>,
    #[serde(rename = "Height")]
    pub height: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode, Eq, PartialEq)]
pub struct Block {
    #[serde(rename = "Messages")]
    pub messages: Cid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode, Eq, PartialEq)]
pub struct Cid {
    #[serde(deserialize_with = "de::bytes")]
    #[serde(rename = "/")]
    pub empty: Vec<u8>,
}
