// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::{
    crypto::{FilecoindotId, KEY_TYPE},
    ocw::offchain_worker,
    pallet::*,
    result::{Error, Result},
};

mod api;
mod crypto;
mod de;
mod ocw;
mod result;
mod types;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        log,
        pallet_prelude::{Hooks, IsType, TransactionSource, TransactionValidity},
        sp_runtime::{traits::ValidateUnsigned, transaction_validity::InvalidTransaction},
    };
    use frame_system::{
        offchain::{AppCrypto, CreateSignedTransaction},
        pallet_prelude::BlockNumberFor,
    };

    /// Filecoindot offchain worker config
    #[pallet::config]
    pub trait Config:
        CreateSignedTransaction<Call<Self>> + frame_system::Config + filecoindot::Config
    {
        type Call: From<Call<Self>>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }

    /// Filecoindot offchain worker pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Filecoindot offchain worker hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            if let Err(e) = crate::ocw::offchain_worker::<T>(block_number) {
                log::error!("{}", e);
            }
        }
    }

    /// A public part of the pallet.
    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// Events for the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NewPrice(u32),
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validate unsigned call to this module.
        ///
        /// By default unsigned transactions are disallowed, but implementing the validator
        /// here we make sure that some particular calls (the ones produced by offchain worker)
        /// are being whitelisted and marked as valid.
        fn validate_unsigned(
            _source: TransactionSource,
            _call: &Self::Call,
        ) -> TransactionValidity {
            InvalidTransaction::Call.into()
        }
    }
}
