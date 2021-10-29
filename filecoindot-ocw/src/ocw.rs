// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! offchain worker logics
use crate::{
    result::{Error, Result},
    Config,
};
use frame_support::{log, sp_runtime::offchain::storage::StorageValueRef, sp_std::vec::Vec};
use frame_system::offchain::Signer;

/// the storage key of filecoin rpc endpoint
pub const FILECOIN_RPC: &'static [u8] = b"FILECOIN_RPC";

/// offchain worker entry
pub fn offchain_worker<T: Config>(block_number: T::BlockNumber) -> Result<()> {
    if let Some(url) = StorageValueRef::local(FILECOIN_RPC)
        .get::<Vec<u8>>()
        .map_err(|_| Error::FilecoinRpcNotSet)?
    {
        log::info!("\"hello, world\" from filecoindot ocw!");
        log::info!("bootstrap relayer with filecoin rpc {}", unsafe {
            core::str::from_utf8_unchecked(&url)
        });

        // log errors from ocw
        bootstrap::<T>(block_number)?;
    }

    Ok(())
}

/// bootstrap filcoindot ocw
fn bootstrap<T: Config>(_: T::BlockNumber) -> Result<()> {
    let _signer = Signer::<T, T::AuthorityId>::all_accounts();

    Ok(())
}
