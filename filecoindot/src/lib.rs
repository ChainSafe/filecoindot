// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! # Filecoin Governance Pallet
//!
//! This pallet uses a set of AccountIds to identify who
//! can vote on proposals. Relayers may be added, removed.
//! There is no bound on how many members may exist in the committee.
//!
//! For each block addition proposal, relayers can vote on them.
//! The pallet will lazily resolve all the proposals.
//! Admin could also resolve manually.
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::{
    crypto::{FilecoindotId, KEY_TYPE},
    pallet::*,
};

mod crypto;
mod ocw;
#[cfg(test)]
mod tests;
mod types;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        log,
        pallet_prelude::*,
        sp_runtime::{
            traits::{Saturating, ValidateUnsigned},
            transaction_validity::InvalidTransaction,
        },
        sp_std::prelude::*,
    };
    use frame_system::{
        offchain::{AppCrypto, CreateSignedTransaction},
        pallet_prelude::*,
    };

    use crate::types::{BlockSubmissionProposal, ProposalStatus};
    use filecoindot_io::forest_proof_verify;

    pub(crate) const DEFAULT_VOTE_THRESHOLD: u32 = 1;

    pub(crate) type BlockSubmissionProposalOf<T> = BlockSubmissionProposal<
        <T as frame_system::Config>::AccountId,
        <T as frame_system::Config>::BlockNumber,
    >;

    // TODO: clarify the exact type, too many clones
    pub(crate) type BlockCid = Vec<u8>;

    // TODO: clarify the exact type, too many clones
    pub(crate) type MessageRootCid = Vec<u8>;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
        /// The identifier type for the offchain worker.
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Origin used to administer the pallet
        type ManagerOrigin: EnsureOrigin<Self::Origin>;
        /// The weight for this pallet's extrinsics.
        type WeightInfo: WeightInfo;
        /// The timeout of the http requests of ocw in milliseconds
        type OffchainWorkerTimeout: Get<u64>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Track the account id of each relayer
    #[pallet::storage]
    pub(crate) type Relayers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (), OptionQuery>;

    /// Count the total number of relayers
    #[pallet::storage]
    pub(super) type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Track the block submission related proposals stored
    #[pallet::storage]
    pub(crate) type BlockSubmissionProposals<T: Config> =
        StorageMap<_, Blake2_128Concat, BlockCid, BlockSubmissionProposalOf<T>, OptionQuery>;

    /// Track the accounts which voted for a particular submitted block proposal
    #[pallet::storage]
    pub(crate) type BlockProposalVotes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        BlockCid,
        Blake2_128Concat,
        T::AccountId,
        (),
        OptionQuery,
    >;

    /// Track the message root cid votes for block cid
    #[pallet::storage]
    pub(crate) type MessageRootCidCounter<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        BlockCid,
        Blake2_128Concat,
        MessageRootCid,
        u32,
        OptionQuery,
    >;

    /// Track the blocks that have been verified
    #[pallet::storage]
    pub(crate) type VerifiedBlocks<T: Config> =
        StorageMap<_, Blake2_128Concat, BlockCid, (), OptionQuery>;

    /// The threshold of votes required for a proposal to be qualified for approval resolution
    #[pallet::storage]
    pub(super) type VoteThreshold<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The voting period of a proposal
    #[pallet::storage]
    pub(super) type VotingPeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Relayer added to set
        /// \[AccountId\]
        RelayerAdded(T::AccountId),
        /// Relayer removed from set
        /// \[AccountId\]
        RelayerRemoved(T::AccountId),
        /// Relayer threshold updated to value
        /// \[RelayerThreshold\]
        VoteThresholdChanged(u32),
        /// Vote for the proposal casted
        /// \[BlockCid, AccountId\]
        VoteCasted(BlockCid, T::AccountId),
        /// The proposal is approved
        /// \[BlockCid\]
        ProposalApproved(BlockCid),
        /// The proposal is rejected
        /// \[BlockCid\]
        ProposalRejected(BlockCid),
        /// Vote against the proposal casted
        /// \[BlockCid, AccountId\]
        VoteAgainstCasted(BlockCid, T::AccountId),
        /// Proposal created
        /// \[BlockCid\]
        ProposalCreated(BlockCid),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Invalid threshold
        InvalidThreshold,
        /// Relayer already in set
        RelayerAlreadyExists,
        /// Provided accountId is not a relayer
        NotRelayer,
        /// Not enough relayers
        NotEnoughRelayer,
        /// Proposal has already completed
        ProposalCompleted,
        /// Proposal has already expired
        ProposalExpired,
        /// Proposal does not exist
        ProposalNotExists,
        /// Relayer has already voted
        AlreadyVoted,
        /// The block has already been verified
        BlockAlreadyVerified,
        /// Cannot verify the proof provided
        VerificationError,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            if let Err(e) = crate::ocw::offchain_worker::<T>(block_number) {
                log::error!("{}", e);
            }
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub vote_threshold: u32,
        pub vote_period: T::BlockNumber,
        /// The initial number of relayers
        pub relayers: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                vote_threshold: DEFAULT_VOTE_THRESHOLD,
                vote_period: Default::default(),
                relayers: Default::default(),
            }
        }
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

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            VoteThreshold::<T>::put(self.vote_threshold);
            VotingPeriod::<T>::put(self.vote_period);
            for r in self.relayers.clone() {
                // should not fail in this case
                Pallet::<T>::register_relayer(r).unwrap();
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // **************** Relayer Add/Remove *****************
        /// Adds a new relayer to the relayer set.
        #[pallet::weight(T::WeightInfo::add_relayer())]
        pub fn add_relayer(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::register_relayer(v)
        }

        /// Removes an existing relayer from the set.
        #[pallet::weight(T::WeightInfo::remove_relayer())]
        pub fn remove_relayer(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
            Self::ensure_admin(origin)?;
            Self::unregister_relayer(v)
        }

        /// Update the vote threshold
        #[pallet::weight(T::WeightInfo::set_vote_threshold())]
        pub fn set_vote_threshold(origin: OriginFor<T>, threshold: u32) -> DispatchResult {
            Self::ensure_admin(origin)?;
            ensure!(
                threshold > 0 && threshold <= RelayerCount::<T>::get(),
                Error::<T>::InvalidThreshold
            );
            VoteThreshold::<T>::set(threshold);

            Self::deposit_event(Event::VoteThresholdChanged(threshold));
            Ok(())
        }

        // ************** Proposal Lifecycle *************
        /// Commits a vote in favour of the provided block cid and message root.
        #[pallet::weight(T::WeightInfo::submit_block_vote())]
        pub fn submit_block_vote(
            origin: OriginFor<T>,
            block_cid: BlockCid,
            message_root_cid: MessageRootCid,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_relayer(&who), Error::<T>::NotRelayer);
            ensure!(
                !VerifiedBlocks::<T>::contains_key(block_cid.clone()),
                Error::<T>::BlockAlreadyVerified
            );

            BlockSubmissionProposals::<T>::try_mutate(
                block_cid.clone(),
                |maybe_proposal| -> Result<(), DispatchError> {
                    let proposal = maybe_proposal.get_or_insert_with(|| {
                        let start_block: T::BlockNumber = frame_system::Pallet::<T>::block_number();
                        let end_block = start_block.saturating_add(VotingPeriod::<T>::get());
                        let r = BlockSubmissionProposal::new(who.clone(), start_block, end_block);
                        Self::deposit_event(Event::ProposalCreated(block_cid.clone()));
                        r
                    });

                    match Self::vote_block_proposal(
                        block_cid.clone(),
                        message_root_cid,
                        proposal,
                        who.clone(),
                    ) {
                        Ok(()) => {
                            Self::deposit_event(Event::VoteCasted(block_cid.clone(), who));
                            if Self::try_resolve_proposal(block_cid, proposal) {
                                *maybe_proposal = None;
                            }
                            Ok(())
                        }
                        Err(e) => match e {
                            // Resolution is performed lazily, if it happens to be expired,
                            // we will issue resolution command.
                            Error::<T>::ProposalExpired => {
                                if Self::try_resolve_proposal(block_cid, proposal) {
                                    *maybe_proposal = None;
                                }
                                Err(e.into())
                            }
                            e => Err(e.into()),
                        },
                    }
                },
            )?;

            Ok(())
        }

        /// Admin can close the proposal when it has expired. The admin ought to have called this
        /// when the proposal expired, otherwise it
        #[pallet::weight(T::WeightInfo::close_block_proposal())]
        pub fn close_block_proposal(origin: OriginFor<T>, block_cid: BlockCid) -> DispatchResult {
            Self::ensure_admin(origin)?;
            ensure!(
                !VerifiedBlocks::<T>::contains_key(block_cid.clone()),
                Error::<T>::BlockAlreadyVerified
            );

            let mut p = BlockSubmissionProposals::<T>::get(&block_cid)
                .ok_or(Error::<T>::ProposalNotExists)?;

            let now = frame_system::Pallet::<T>::block_number();
            let threshold = VoteThreshold::<T>::get();
            Self::resolve_proposal(&mut p, &block_cid, &now, threshold)?;

            Self::try_resolve_proposal(block_cid, &p);

            Ok(())
        }

        /// Verify the receipt of the filecoin
        #[pallet::weight(T::WeightInfo::verify_receipt())]
        pub fn verify_receipt(
            origin: OriginFor<T>,
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            Self::verify_receipt_inner(proof, block_cid, cid)
        }

        /// Verify the state of the filecoin
        #[pallet::weight(T::WeightInfo::verify_state())]
        pub fn verify_state(
            origin: OriginFor<T>,
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            Self::verify_state_inner(proof, block_cid, cid)
        }

        /// Verify the state of the filecoin
        #[pallet::weight(T::WeightInfo::verify_message())]
        pub fn verify_message(
            origin: OriginFor<T>,
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            Self::verify_message_inner(proof, block_cid, cid)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn verify_receipt_inner(
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            forest_proof_verify::verify_receipt(proof, cid).ok_or(Error::<T>::VerificationError)?;
            Self::verified_block(&block_cid)
                .then(|| ())
                .ok_or(Error::<T>::VerificationError)?;
            Ok(())
        }

        pub fn verify_message_inner(
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            forest_proof_verify::verify_message(proof, cid).ok_or(Error::<T>::VerificationError)?;
            Self::verified_block(&block_cid)
                .then(|| ())
                .ok_or(Error::<T>::VerificationError)?;
            Ok(())
        }

        pub fn verify_state_inner(
            proof: Vec<Vec<u8>>,
            block_cid: BlockCid,
            cid: Vec<u8>,
        ) -> DispatchResult {
            forest_proof_verify::verify_state(proof, cid).ok_or(Error::<T>::VerificationError)?;
            Self::verified_block(&block_cid)
                .then(|| ())
                .ok_or(Error::<T>::VerificationError)?;
            Ok(())
        }

        /// Check if the block cid is already verified
        fn verified_block(block_cid: &BlockCid) -> bool {
            VerifiedBlocks::<T>::contains_key(block_cid)
        }

        fn ensure_admin(o: OriginFor<T>) -> DispatchResult {
            T::ManagerOrigin::try_origin(o)
                .map(|_| ())
                .or_else(ensure_root)?;
            Ok(())
        }

        /// Adds a new relayer to the set.
        /// Caller ensure the invoker has appropriate admin roles
        fn register_relayer(relayer: T::AccountId) -> DispatchResult {
            ensure!(
                !Self::is_relayer(&relayer),
                Error::<T>::RelayerAlreadyExists
            );

            Relayers::<T>::insert(&relayer, ());
            RelayerCount::<T>::mutate(|i| {
                *i = i.saturating_add(1);
                *i
            });

            Self::deposit_event(Event::RelayerAdded(relayer));
            Ok(())
        }

        /// Removes a relayer from the set
        /// Caller ensure the invoker has appropriate admin roles
        fn unregister_relayer(relayer: T::AccountId) -> DispatchResult {
            ensure!(Self::is_relayer(&relayer), Error::<T>::NotRelayer);

            let threshold = VoteThreshold::<T>::get();
            RelayerCount::<T>::try_mutate(|i| -> DispatchResult {
                *i = i.saturating_sub(1);
                ensure!(*i >= threshold, Error::<T>::NotEnoughRelayer);
                Ok(())
            })?;
            Relayers::<T>::remove(&relayer);

            Self::deposit_event(Event::RelayerRemoved(relayer));
            Ok(())
        }

        /// Checks if who is a relayer
        fn is_relayer(who: &T::AccountId) -> bool {
            Relayers::<T>::contains_key(who)
        }

        // ============== Voting Related =============
        /// Vote for the proposal. Will reject the operation if its status is invalid
        /// The content of the vote is actually the message root of the block
        fn vote_block_proposal(
            block_cid: BlockCid,
            message_root_cid: Vec<u8>,
            proposal: &mut BlockSubmissionProposalOf<T>,
            who: T::AccountId,
        ) -> Result<(), Error<T>> {
            ensure!(
                !BlockProposalVotes::<T>::contains_key(block_cid.clone(), who.clone()),
                Error::<T>::AlreadyVoted
            );
            ensure!(
                *proposal.get_status() == ProposalStatus::Active,
                Error::<T>::ProposalCompleted
            );

            let now = frame_system::Pallet::<T>::block_number();

            // when expired, we set the status to be rejected
            if proposal.is_expired(&now) {
                proposal.set_status(ProposalStatus::Rejected);
                return Err(Error::<T>::ProposalExpired);
            }
            let threshold = VoteThreshold::<T>::get();

            let count =
                1 + MessageRootCidCounter::<T>::get(&block_cid, &message_root_cid).unwrap_or(0);
            if count >= threshold {
                proposal.set_status(ProposalStatus::Approved);
            }

            MessageRootCidCounter::<T>::insert(&block_cid, &message_root_cid, count);
            BlockProposalVotes::<T>::insert(block_cid, who, ());

            Ok(())
        }

        pub(crate) fn resolve_proposal(
            proposal: &mut BlockSubmissionProposalOf<T>,
            block_cid: &[u8],
            when: &T::BlockNumber,
            threshold: u32,
        ) -> Result<(), Error<T>> {
            ensure!(
                *proposal.get_status() == ProposalStatus::Active,
                Error::<T>::ProposalCompleted
            );

            // when expired, we set the status to be rejected
            if proposal.is_expired(when) {
                proposal.set_status(ProposalStatus::Rejected);
            } else {
                // MessageRootCidCounter leaked into the struct, well not the best way for encapsulation
                // but works for now, come back later to fix this.
                for (_, count) in MessageRootCidCounter::<T>::iter_prefix(block_cid) {
                    if count >= threshold {
                        proposal.set_status(ProposalStatus::Approved);
                        break;
                    }
                }
            }

            Ok(())
        }

        /// Try to resolve the proposal. If the proposal is resolved, return true, else false
        fn try_resolve_proposal(block_cid: BlockCid, prop: &BlockSubmissionProposalOf<T>) -> bool {
            match prop.get_status() {
                ProposalStatus::Approved => {
                    Self::finalize_block(block_cid);
                    true
                }
                ProposalStatus::Rejected => {
                    Self::reject_block(block_cid);
                    true
                }
                _ => false,
            }
        }

        fn finalize_block(block_cid: BlockCid) {
            BlockSubmissionProposals::<T>::remove(&block_cid);
            BlockProposalVotes::<T>::remove_prefix(&block_cid, None);
            MessageRootCidCounter::<T>::remove_prefix(&block_cid, None);

            VerifiedBlocks::<T>::insert(block_cid.clone(), ());

            Self::deposit_event(Event::ProposalApproved(block_cid));
        }

        fn reject_block(block_cid: BlockCid) {
            BlockSubmissionProposals::<T>::remove(&block_cid);
            MessageRootCidCounter::<T>::remove_prefix(&block_cid, None);

            VerifiedBlocks::<T>::remove(block_cid.clone());

            Self::deposit_event(Event::ProposalRejected(block_cid));
        }
    }

    pub trait WeightInfo {
        fn add_relayer() -> Weight;
        fn remove_relayer() -> Weight;
        fn submit_block_vote() -> Weight;
        fn set_vote_threshold() -> Weight;
        fn close_block_proposal() -> Weight;
        fn verify_receipt() -> Weight;
        fn verify_state() -> Weight;
        fn verify_message() -> Weight;
    }

    /// For backwards compatibility and tests
    impl WeightInfo for () {
        fn add_relayer() -> Weight {
            Default::default()
        }

        fn remove_relayer() -> Weight {
            Default::default()
        }

        fn submit_block_vote() -> Weight {
            Default::default()
        }

        fn set_vote_threshold() -> Weight {
            Default::default()
        }

        fn close_block_proposal() -> Weight {
            Default::default()
        }

        fn verify_receipt() -> Weight {
            Default::default()
        }

        fn verify_state() -> Weight {
            Default::default()
        }

        fn verify_message() -> Weight {
            Default::default()
        }
    }
}
