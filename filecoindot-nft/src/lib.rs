// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! # Filecoin NFT Pallet
//!
//! This pallet will enabled users to mint NFTs
//! providing the cid and proof.
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pallet::*;

mod types;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    pub use crate::types::{ClassData, TokenData};
    use codec::alloc::vec;
    use frame_support::{pallet_prelude::*, sp_std::prelude::*};
    use frame_system::pallet_prelude::*;

    pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
    pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;

    const MINT_QUANTITY: u32 = 1;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + orml_nft::Config<ClassData = ClassData, TokenData = TokenData>
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The default class id of the NFT
        type DefaultClassId: Get<ClassIdOf<Self>>;
        /// The weight for this pallet's extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Minted NFT token. \[from, class_id, quantity\]
        MintedToken(T::AccountId, ClassIdOf<T>, u32),
        /// Transferred NFT token. \[from, to, class_id, token_id\]
        TransferredToken(T::AccountId, T::AccountId, ClassIdOf<T>, TokenIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The requested token id does not exist
        TokenIdNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub default_class: (T::AccountId, Vec<u8>),
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                default_class: (Default::default(), Default::default()),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let (owner, data) = self.default_class.clone();
            // just panic if cannot create class
            orml_nft::Pallet::<T>::create_class(&owner, vec![], ClassData::new(data))
                .expect("cannot create default nft class");
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mint the nft from cid and proof
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(origin: OriginFor<T>, cid: Vec<u8>, proof: Vec<Vec<u8>>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            orml_nft::Pallet::<T>::mint(
                &who,
                T::DefaultClassId::get(),
                vec![],
                TokenData::new(cid, proof),
            )?;
            Self::deposit_event(Event::MintedToken(
                who,
                T::DefaultClassId::get(),
                MINT_QUANTITY,
            ));
            Ok(())
        }

        /// Transfer the nft specified with `token_id` from the sender to `to` account
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            token_id: TokenIdOf<T>,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            orml_nft::Pallet::<T>::tokens(T::DefaultClassId::get(), token_id)
                .ok_or(Error::<T>::TokenIdNotFound)?;
            orml_nft::Pallet::<T>::transfer(&from, &to, (T::DefaultClassId::get(), token_id))?;
            Self::deposit_event(Event::TransferredToken(
                from,
                to,
                T::DefaultClassId::get(),
                token_id,
            ));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the balance of the account
        pub fn balance(who: &T::AccountId) -> u128 {
            orml_nft::TokensByOwner::<T>::iter_prefix((who, T::DefaultClassId::get())).count()
                as u128
        }

        /// Get the list of token ids owned by the account.
        /// Quite expensive, invoke with care. Should use indexer for this.
        pub fn tokens(who: &T::AccountId) -> Vec<TokenIdOf<T>> {
            orml_nft::TokensByOwner::<T>::iter_prefix((who, T::DefaultClassId::get()))
                .map(|t| t.0)
                .collect::<Vec<_>>()
        }

        /// Get the details of a specific token
        pub fn token_detail(token_id: TokenIdOf<T>) -> Result<TokenData, Error<T>> {
            let token = orml_nft::Pallet::<T>::tokens(T::DefaultClassId::get(), token_id)
                .ok_or(Error::<T>::TokenIdNotFound)?;
            Ok(token.data)
        }
    }

    pub trait WeightInfo {
        fn mint() -> Weight;
        fn transfer() -> Weight;
    }

    /// For backwards compatibility and tests
    impl WeightInfo for () {
        fn mint() -> Weight {
            Default::default()
        }
        fn transfer() -> Weight {
            Default::default()
        }
    }
}
