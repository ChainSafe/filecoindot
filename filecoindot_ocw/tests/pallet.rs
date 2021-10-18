// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{log, pallet_prelude::Hooks};
    use frame_system::{offchain::CreateSignedTransaction, pallet_prelude::BlockNumberFor};

    /// Filecoindot offchain worker config
    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {}

    /// Filecoindot offchain worker pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Filecoindot offchain worker hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(_block_number: T::BlockNumber) {
            log::info!("filecoindot offchain worker has been initialized!");

            // TODO
            //
            // 0. make condition about relaying which filecoin blocks
            // 1. relay blocks
        }
    }
}
