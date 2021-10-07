#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Saturating;
    use frame_system::pallet_prelude::*;

    use crate::types::{BlockSubmissionProposal, ProposalStatus, VoteType};

    // TODO: clarify the exact type, too many clones
    type BlockCid = Vec<u8>;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Origin used to administer the pallet
        type ManagerOrigin: EnsureOrigin<Self::Origin>;
        /// The weight for this pallet's extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Track the account id of each relayer
    #[pallet::storage]
    pub type Relayers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, OptionQuery>;

    /// Track the block submission related proposals stored
    #[pallet::storage]
    pub(crate) type BlockSubmissionProposals<T: Config> =
        StorageMap<_, Blake2_128Concat, BlockCid, BlockSubmissionProposal<T>, OptionQuery>;

    /// Track the blocks that have been verified
    #[pallet::storage]
    pub(crate) type VerifiedBlocks<T: Config> =
        StorageMap<_, Blake2_128Concat, BlockCid, bool, ValueQuery>;

    /// Count the total number of relayers
    #[pallet::storage]
    pub(super) type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The threshold of votes required for a proposal to be qualified for approval resolution
    #[pallet::storage]
    pub(super) type VoteThreshold<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The voting period of a proposal
    #[pallet::storage]
    pub(super) type VotingPeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
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
        VoteForCasted(BlockCid, T::AccountId),
        /// The proposal is approved
        /// \[BlockCid\]
        ProposalApproved(BlockCid),
        /// The proposal is executed
        /// \[BlockCid\]
        ProposalExecuted(BlockCid),
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
        /// Relayer already in set
        RelayerAlreadyExists,
        /// Provided accountId is not a relayer
        RelayerInvalid,
        /// Protected operation, must be performed by relayer
        MustBeRelayer,
        /// Proposal has already executed
        ProposalAlreadyExecuted,
        /// Proposal has already expired
        ProposalExpired,
        /// Proposal invalid status
        ProposalInvalidStatus,
        /// Proposal already exists
        ProposalAlreadyExists,
        /// Proposal does not exist
        ProposalNotExists,
        /// Relayer has already voted
        AlreadyVoted,
        /// The block has already been verified
        BlockAlreadyVerified,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

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

        /// Commits a vote in favour of the provided proposal.
        #[pallet::weight(T::WeightInfo::set_vote_threshold())]
        pub fn set_vote_threshold(origin: OriginFor<T>, threshold: u32) -> DispatchResult {
            Self::ensure_admin(origin)?;
            ensure!(
                threshold > 0 && threshold < RelayerCount::<T>::get(),
                "Invalid threshold"
            );
            VoteThreshold::<T>::set(threshold);

            Self::deposit_event(Event::VoteThresholdChanged(threshold));
            Ok(())
        }

        // ************** Proposal Lifecycle *************
        /// TODO: who and when can create proposals?
        #[pallet::weight(T::WeightInfo::submit_block_vote())]
        pub fn submit_block_vote(
            origin: OriginFor<T>,
            block_cid: Vec<u8>,
            message_root_cid: Vec<u8>,
            vote_type: VoteType,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_relayer(&who), Error::<T>::MustBeRelayer);
            ensure!(
                !VerifiedBlocks::<T>::contains_key(block_cid.clone()),
                Error::<T>::BlockAlreadyVerified
            );

            if !BlockSubmissionProposals::<T>::contains_key(block_cid.clone()) {
                let start_block: T::BlockNumber = <frame_system::Pallet<T>>::block_number();
                let end_block = start_block.saturating_add(VotingPeriod::<T>::get());
                let proposal = BlockSubmissionProposal::new(
                    who.clone(),
                    message_root_cid,
                    start_block,
                    end_block,
                );
                BlockSubmissionProposals::<T>::insert(block_cid.clone(), proposal);
                Self::deposit_event(Event::ProposalCreated(block_cid.clone()));
            }

            Self::vote_block_proposal(block_cid, who, vote_type)
        }
    }

    impl<T: Config> Pallet<T> {
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

            Relayers::<T>::insert(&relayer, true);
            RelayerCount::<T>::mutate(|i| (*i).saturating_add(1));

            Self::deposit_event(Event::RelayerAdded(relayer));
            Ok(())
        }

        /// Removes a relayer from the set
        /// Caller ensure the invoker has appropriate admin roles
        fn unregister_relayer(relayer: T::AccountId) -> DispatchResult {
            ensure!(Self::is_relayer(&relayer), Error::<T>::RelayerInvalid);

            Relayers::<T>::remove(&relayer);
            RelayerCount::<T>::mutate(|i| (*i).saturating_sub(1));

            Self::deposit_event(Event::RelayerRemoved(relayer));
            Ok(())
        }

        /// Checks if who is a relayer
        fn is_relayer(who: &T::AccountId) -> bool {
            Relayers::<T>::get(who).unwrap_or(false)
        }

        // ============== Voting Related =============
        pub fn vote_block_proposal(
            block_cid: Vec<u8>,
            who: T::AccountId,
            vote_type: VoteType,
        ) -> DispatchResult {
            ensure!(Self::is_relayer(&who), Error::<T>::MustBeRelayer);

            let mut proposal = BlockSubmissionProposals::<T>::get(block_cid.clone())
                .ok_or(Error::<T>::ProposalNotExists)?;

            let now = <frame_system::Pallet<T>>::block_number();
            proposal.vote(who.clone(), &now, &vote_type)?;

            match vote_type {
                VoteType::For => Self::deposit_event(Event::VoteForCasted(block_cid.clone(), who)),
                VoteType::Against => {
                    Self::deposit_event(Event::VoteAgainstCasted(block_cid.clone(), who))
                }
            }

            Self::try_resolve_proposal(block_cid, proposal)
        }

        fn try_resolve_proposal(
            block_cid: BlockCid,
            prop: BlockSubmissionProposal<T>,
        ) -> DispatchResult {
            let now = <frame_system::Pallet<T>>::block_number();

            let threshold = VoteThreshold::<T>::get();
            match prop.status(&now, threshold) {
                ProposalStatus::Approved => Self::finalize_execution(block_cid),
                ProposalStatus::Rejected => Self::cancel_execution(block_cid),
                _ => Ok(()),
            }
        }

        fn finalize_execution(block_cid: BlockCid) -> DispatchResult {
            Self::deposit_event(Event::ProposalApproved(block_cid.clone()));

            BlockSubmissionProposals::<T>::remove(&block_cid);
            VerifiedBlocks::<T>::insert(block_cid.clone(), true);

            Self::deposit_event(Event::ProposalExecuted(block_cid));

            Ok(())
        }

        fn cancel_execution(block_cid: BlockCid) -> DispatchResult {
            BlockSubmissionProposals::<T>::remove(&block_cid);
            VerifiedBlocks::<T>::insert(block_cid.clone(), false);
            Self::deposit_event(Event::ProposalRejected(block_cid));
            Ok(())
        }
    }

    pub trait WeightInfo {
        fn add_relayer() -> Weight;
        fn remove_relayer() -> Weight;
        fn submit_block_vote() -> Weight;
        fn set_vote_threshold() -> Weight;
        fn new_submission() -> Weight;
        fn close_block_submission() -> Weight;
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

        fn new_submission() -> Weight {
            Default::default()
        }

        fn close_block_submission() -> Weight {
            Default::default()
        }
    }
}
