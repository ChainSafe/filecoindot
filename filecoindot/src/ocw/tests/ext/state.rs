// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use reqwest::{Client, RequestBuilder};
use std::collections::BTreeMap;

/// Request with resp body and reading ptr
pub struct Request {
    pub req: RequestBuilder,
    pub resp: Response,
    pub read: usize,
}

impl From<RequestBuilder> for Request {
    fn from(req: RequestBuilder) -> Self {
        Self {
            req,
            resp: Default::default(),
            read: 0,
        }
    }
}

#[derive(Default)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    pub body: Vec<u8>,
}

/// filecoindot offchain state
#[derive(Default)]
pub struct OffchainState {
    pub counter: u16,
    pub client: Client,
    pub db: BTreeMap<Vec<u8>, Vec<u8>>,
    pub requests: BTreeMap<u16, Request>,
}
