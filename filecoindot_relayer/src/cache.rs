// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! local storage for handling cache
use crate::{api::Api, Error, Result};
use rocksdb::DB;

const FILECOINDOT_CACHE_FILE: &str = "filecoindot_cache";

/// api requests cache
pub struct Cache(DB);

impl Cache {
    /// New cache with data dir
    ///
    /// | OS      | dir                                   |
    /// | ------- | ------------------------------------- |
    /// | Linux   | $XDG_DATA_HOME or $HOME/.local/share  |
    /// | macOS   | $HOME/Library/Application Support     |
    /// | Windows | {FOLDERID_RoamingAppData}             |
    pub fn new() -> Result<Self> {
        Ok(Self(DB::open_default(
            dirs::data_dir()
                .ok_or(Error::DirectoryNotFound)?
                .join(FILECOINDOT_CACHE_FILE),
        )?))
    }

    /// Get api cache from cache
    pub fn get<A: Api>(&self, params: &A::Params) -> Result<Option<Vec<u8>>> {
        Ok(self.0.get(<A as Api>::storage_key(params)?)?)
    }

    /// Put api cache to cache
    pub fn put<A: Api>(&self, params: &A::Params, value: &[u8]) -> Result<()> {
        Ok(self.0.put(<A as Api>::storage_key(params)?, value)?)
    }
}
