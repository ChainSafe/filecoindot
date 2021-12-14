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
    sp_runtime::offchain::{storage::StorageValueRef, Timestamp},
    sp_std::vec::Vec,
    traits::Get,
};
use frame_system::offchain::{SendSignedTransaction, Signer};

pub mod api;
mod de;
mod result;
pub mod types;

#[cfg(test)]
mod tests;

/// the storage key of filecoin rpc endpoint
pub const FILECOIN_RPC: &[u8] = b"FILECOIN_RPC";

/// offchain worker entry
pub fn offchain_worker<T: Config>(block_number: T::BlockNumber) -> Result<()> {
    // get encoded urls from storage
    let urls = StorageValueRef::persistent(FILECOIN_RPC)
        .get::<Vec<u8>>()
        .map_err(|_| Error::GetStorageFailed)?
        .ok_or(Error::FilecoinRpcNotSet)?;

    // decode endpoints
    let endpoints = core::str::from_utf8(&urls)
        .map_err(|_| Error::FormatBytesFailed)?
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    // check if endpoints is empty
    if endpoints.is_empty() {
        return Err(Error::FilecoinRpcNotSet);
    }

    // bootstrap ocw
    bootstrap::<T>(block_number, &endpoints)?;

    Ok(())
}

/// bootstrap filcoindot ocw
fn bootstrap<T: Config>(_: T::BlockNumber, urls: &[&str]) -> Result<()> {
    let signer = Signer::<T, T::AuthorityId>::any_account();
    vote_on_chain_head(signer, urls)
}

fn vote_on_chain_head<T: Config>(signer: Signer<T, T::AuthorityId>, urls: &[&str]) -> Result<()> {
    let pairs = ChainHead
        .iter_req(
            urls,
            Vec::new(),
            Timestamp::from_unix_millis(T::OffchainWorkerTimeout::get()),
        )
        .map_err(|_| Error::HttpError)?
        .pairs()?;

    if pairs
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
    {
        return Err(Error::OffchainSignedTxError);
    }

    Ok(())
}
