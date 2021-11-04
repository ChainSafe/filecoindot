// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! offchain worker logics
use crate::{
    api::{Api, ChainHeight},
    result::{Error, Result},
    Config,
};
use filecoindot::Call;
use frame_support::{log, sp_runtime::offchain::storage::StorageValueRef, sp_std::vec::Vec};
use frame_system::offchain::{SendSignedTransaction, Signer};

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
    let pairs = ChainHeight
        .req(url, Default::default())
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
