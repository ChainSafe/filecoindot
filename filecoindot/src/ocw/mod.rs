// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use crate::{
    crypto::FilecoindotId,
    ocw::{
        api::{Api, ChainHead},
        result::{Error, Result},
    },
    Call, Config, Relayers,
};
use frame_support::{
    codec::Encode,
    sp_runtime::{
        offchain::{storage::StorageValueRef, Timestamp},
        traits::Verify,
        RuntimeAppPublic,
    },
    sp_std::vec::Vec,
    traits::Get,
};
use frame_system::offchain::{SendSignedTransaction, Signer};
use sp_core::sr25519::Signature as Sr25519Signature;

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
        .get::<Vec<Vec<u8>>>()
        .map_err(|_| Error::GetStorageFailed)?
        .ok_or(Error::FilecoinRpcNotSet)?;

    // decode endpoints
    let endpoints: Vec<&str> = urls
        .iter()
        .map(|url_bytes| core::str::from_utf8(url_bytes))
        .collect::<core::result::Result<Vec<_>, _>>()
        .map_err(|_| Error::FormatBytesFailed)?;

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
    let all_public: Vec<Vec<u8>> = <FilecoindotId as frame_system::offchain::AppCrypto<
        <Sr25519Signature as Verify>::Signer,
        Sr25519Signature,
    >>::RuntimeAppPublic::all()
    .into_iter()
    .map(|key| key.encode())
    .collect();

    // check if keystore has key listed in Relayers
    let mut no_relayer = true;
    if Relayers::<T>::iter().any(|relayer| all_public.contains(&relayer.encode())) {
        no_relayer = false;
    }

    let signer = Signer::<T, T::AuthorityId>::any_account();
    if !signer.can_sign() || no_relayer {
        return Err(Error::NoRelayerFound);
    }

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
                .send_signed_transaction(|_| Call::submit_block_vote {
                    block_cid: cid.to_vec(),
                    message_root_cid: msg_root.to_vec(),
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
