// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! offchain worker logics
use crate::{
    result::{Error, Result},
    Config,
};
use filecoindot::Call;
use frame_support::{log, sp_runtime::offchain::storage::StorageValueRef, sp_std::vec::Vec};
use frame_system::offchain::Signer;

/// the storage key of filecoin rpc endpoint
pub const FILECOIN_RPC: &[u8] = b"FILECOIN_RPC";

/// offchain worker entry
pub fn offchain_worker<T: Config>(block_number: T::BlockNumber) -> Result<()> {
    let url = StorageValueRef::persistent(FILECOIN_RPC)
        .get::<Vec<u8>>()
        .map_err(|_| Error::GetStorageFailed)?
        .ok_or(Error::FilecoinRpcNotSet)?;

    // log out filecoin rpc endpoint
    log::info!(
        "bootstrap filecoindot ocw with filecoin rpc endpoint {}",
        core::str::from_utf8(&url).map_err(|_| Error::FormatBytesFailed)?
    );

    // log errors from ocw
    bootstrap::<T>(block_number, &url)?;

    Ok(())
}

/// bootstrap filcoindot ocw
fn bootstrap<T: Config>(_: T::BlockNumber, _: &[u8]) -> Result<()> {
    let signer = Signer::<T, T::AuthorityId>::all_accounts();
    frame_support::sp_std::if_std! {
        println!("ability to sign {:?}", signer.can_sign());
    }

    Ok(())
}
