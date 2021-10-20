// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueMultiaddr, OpaqueNetworkState, Timestamp,
};
use reqwest::{
    header::HeaderName,
    Method, Url, {Body, Client, Request, Response},
};
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Default)]
pub struct OffchainState {
    pub counter: u16,
    pub client: Client,
    pub requests: BTreeMap<u16, (Request, Option<Response>)>,
}
