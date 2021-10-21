// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Error, Result};
use reqwest::{Client, RequestBuilder};
use rocksdb::DB;
use std::collections::BTreeMap;

const FILECOINDOT_CACHE_FILE: &str = "filecoindot_cache";

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
pub struct OffchainState {
    pub counter: u16,
    pub client: Client,
    pub db: DB,
    pub requests: BTreeMap<u16, Request>,
}

impl OffchainState {
    /// New offchain state
    ///
    /// # database directory
    ///
    /// | OS      | dir                                   |
    /// | ------- | ------------------------------------- |
    /// | Linux   | $XDG_DATA_HOME or $HOME/.local/share  |
    /// | macOS   | $HOME/Library/Application Support     |
    /// | Windows | {FOLDERID_RoamingAppData}             |
    pub fn new() -> Result<Self> {
        let db = DB::open_default(
            dirs::data_dir()
                .ok_or(Error::DirectoryNotFound)?
                .join(FILECOINDOT_CACHE_FILE),
        )?;
        Ok(Self {
            client: Default::default(),
            counter: Default::default(),
            db,
            requests: Default::default(),
        })
    }
}
