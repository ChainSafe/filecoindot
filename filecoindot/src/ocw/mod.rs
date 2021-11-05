// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use crate::{
    ocw::{
        api::{Api, ChainHead},
        result::{Error, Result},
    },
    Call, Config,
};
use frame_support::{
    log,
    sp_runtime::offchain::{storage::StorageValueRef, Timestamp},
    sp_std::vec::Vec,
    traits::Get,
};
use frame_system::offchain::{SendSignedTransaction, Signer};

mod api;
mod de;
mod result;
mod types;

#[cfg(test)]
mod tests;

/// the storage key of filecoin rpc endpoint
pub const FILECOIN_RPC: &[u8] = b"FILECOIN_RPC";

/// offchain worker entry
pub fn offchain_worker<T: Config>(block_number: T::BlockNumber) -> Result<()> {
    let url = StorageValueRef::persistent(FILECOIN_RPC)
        .get::<Vec<u8>>()
        .map_err(|_| Error::GetStorageFailed)?
        .ok_or(Error::FilecoinRpcNotSet)?;

    // log out filecoin rpc endpoint
    let url_str = core::str::from_utf8(&url).map_err(|_| Error::FormatBytesFailed)?;
    log::info!(
        "bootstrap filecoindot ocw with filecoin rpc endpoint {}",
        url_str
    );

    // log errors from ocw
    bootstrap::<T>(block_number, url_str)?;

    Ok(())
}

/// bootstrap filcoindot ocw
fn bootstrap<T: Config>(_: T::BlockNumber, url: &str) -> Result<()> {
    let signer = Signer::<T, T::AuthorityId>::any_account();
    vote_on_chain_head(signer, url)
}

fn vote_on_chain_head<T: Config>(signer: Signer<T, T::AuthorityId>, url: &str) -> Result<()> {
    let pairs = ChainHead
        .req(
            url,
            Default::default(),
            Timestamp::from_unix_millis(T::OffchainWorkerTimeout::get()),
        )
        .map_err(|_| Error::HttpError)?
        .pairs()?;

    pairs
        .into_iter()
        .map(|(cid, msg_root)| {
            // FIXME:
            //
            // still requires taking the ownership even under `into_iter()`
            let (_, res) = signer
                .send_signed_transaction(|_| {
                    Call::submit_block_vote(cid.to_vec(), msg_root.to_vec())
                })
                .ok_or(Error::NoTxResult)?;

            let _ = res.map_err(|_| Error::OffchainSignedTxError)?;
            Ok(())
        })
        .any(|x| x == Err(Error::OffchainSignedTxError))
        .then(|| Some(()))
        .ok_or(Error::OffchainSignedTxError)?;

    Ok(())
}
