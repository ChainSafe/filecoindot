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
    use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::{AccountIdConversion, Saturating};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;

    use crate::types::{GovernanceProposal, ProposalStatus, VoteType};

    type ProposalId<T> = <T as frame_system::Config>::Hash;

    const PALLET_ID: PalletId = PalletId(*b"cb/subfi");

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Origin used to administer the pallet
        /// TODO: Do we need dynamically change admin here?
        type ManagerOrigin: EnsureOrigin<Self::Origin>;
        /// The outer call dispatch type.
        type Action: Parameter
            + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
            + From<frame_system::Call<Self>>
            + GetDispatchInfo;
        /// Origin that is permitted to create proposals
        type ProposalSubmissionOrigin: EnsureOrigin<
            <Self as frame_system::Config>::Origin,
            Success = <Self as frame_system::Config>::AccountId,
        >;
        /// Origin that is permitted to execute approved proposals
        type ProposalExecutionOrigin: EnsureOrigin<
            <Self as frame_system::Config>::Origin,
            Success = <Self as frame_system::Config>::AccountId,
        >;
        /// The lifetime of a proposal
        type ProposalLifetime: Get<Self::BlockNumber>;
        /// The weight for this pallet's extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Track the account id of each relayer
    #[pallet::storage]
    pub type Relayers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, OptionQuery>;

    /// Track the governance related proposals stored
    #[pallet::storage]
    pub(crate) type GovernanceProposals<T: Config> =
        StorageMap<_, Blake2_128Concat, ProposalId<T>, GovernanceProposal<T>, OptionQuery>;

    /// Count the total number of relayers
    #[pallet::storage]
    pub(super) type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The threshold of votes required for a proposal to be qualified for approval resolution
    #[pallet::storage]
    pub(super) type RelayerThreshold<T: Config> = StorageValue<_, u32, ValueQuery>;

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
        RelayerThresholdUpdate(u32),
        /// Vote for the proposal casted
        /// \[ProposalId, AccountId\]
        VoteForCasted(ProposalId<T>, T::AccountId),
        /// The proposal is approved
        /// \[ProposalId\]
        ProposalApproved(ProposalId<T>),
        /// The proposal is executed
        /// \[ProposalId\]
        ProposalExecuted(ProposalId<T>),
        /// The proposal is rejected
        /// \[ProposalId\]
        ProposalRejected(ProposalId<T>),
        /// Vote against the proposal casted
        /// \[ProposalId, AccountId\]
        VoteAgainstCasted(ProposalId<T>, T::AccountId),
        /// Proposal created
        /// \[ProposalId\]
        ProposalCreated(ProposalId<T>),
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
        #[pallet::weight(T::WeightInfo::update_relayer_threshold())]
        pub fn update_relayer_threshold(origin: OriginFor<T>, threshold: u32) -> DispatchResult {
            Self::ensure_admin(origin)?;
            ensure!(
                threshold > 0 && threshold < RelayerCount::<T>::get(),
                "Invalid threshold"
            );
            RelayerThreshold::<T>::set(threshold);

            Self::deposit_event(Event::RelayerThresholdUpdate(threshold));
            Ok(())
        }

        // ************** Proposal Lifecycle *************
        /// TODO: who and when can create proposals?
        #[pallet::weight(T::WeightInfo::new_submission())]
        pub fn new_submission(
            _origin: OriginFor<T>,
            _block_cid: Vec<u8>,
            _message_root_cid: Vec<u8>,
        ) -> DispatchResult {
            todo!()
        }

        /// TODO: who and when can create proposals?
        #[pallet::weight(T::WeightInfo::new_governance_proposal())]
        pub fn new_governance_proposal(origin: OriginFor<T>, action: T::Action) -> DispatchResult {
            let who = T::ProposalSubmissionOrigin::ensure_origin(origin)?;

            let start_block: T::BlockNumber = <frame_system::Pallet<T>>::block_number();
            let end_block = start_block.saturating_add(VotingPeriod::<T>::get());
            let proposal: GovernanceProposal<T> =
                GovernanceProposal::new(action, who, start_block, end_block);

            let proposal_id: ProposalId<T> = proposal.hash();
            if GovernanceProposals::<T>::contains_key(proposal_id) {
                return Err(Error::<T>::ProposalAlreadyExists.into());
            }
            GovernanceProposals::<T>::insert(proposal_id, proposal);
            Self::deposit_event(Event::ProposalCreated(proposal_id));

            Ok(())
        }

        // **************** Voting Related ***************
        /// Vote for a provided proposal.
        #[pallet::weight(T::WeightInfo::vote_governance_proposal())]
        pub fn vote_governance_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId<T>,
            vote_type: VoteType,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_relayer(&who), Error::<T>::MustBeRelayer);

            let mut proposal =
                GovernanceProposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExists)?;

            let now = <frame_system::Pallet<T>>::block_number();
            proposal.vote(who.clone(), &now, &vote_type)?;

            match vote_type {
                VoteType::For => Self::deposit_event(Event::VoteForCasted(proposal_id, who)),
                VoteType::Against => {
                    Self::deposit_event(Event::VoteAgainstCasted(proposal_id, who))
                }
            }

            Self::try_resolve_proposal(proposal_id, proposal)
        }

        /// Close a governance proposal in case auto closing didn't complete
        #[pallet::weight(T::WeightInfo::close_governance_proposal())]
        pub fn close_governance_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId<T>,
        ) -> DispatchResult {
            T::ProposalExecutionOrigin::ensure_origin(origin)?;
            let proposal =
                GovernanceProposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExists)?;
            Self::try_resolve_proposal(proposal_id, proposal)
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
        fn try_resolve_proposal(
            prop_id: ProposalId<T>,
            prop: GovernanceProposal<T>,
        ) -> DispatchResult {
            let now = <frame_system::Pallet<T>>::block_number();

            let threshold = RelayerThreshold::<T>::get();
            match prop.status(&now, threshold) {
                ProposalStatus::Approved => Self::finalize_execution(prop_id, prop),
                ProposalStatus::Rejected => Self::cancel_execution(prop_id, prop),
                _ => Ok(()),
            }
        }

        fn finalize_execution(
            prop_id: ProposalId<T>,
            mut prop: GovernanceProposal<T>,
        ) -> DispatchResult {
            Self::deposit_event(Event::ProposalApproved(prop_id));
            prop.action()
                .dispatch(frame_system::RawOrigin::Signed(Self::account_id()).into())
                .map(|_| ())
                .map_err(|e| e.error)?;

            prop.set_executed();
            GovernanceProposals::<T>::insert(prop_id, prop);

            Self::deposit_event(Event::ProposalExecuted(prop_id));

            Ok(())
        }

        fn cancel_execution(
            prop_id: ProposalId<T>,
            mut prop: GovernanceProposal<T>,
        ) -> DispatchResult {
            prop.set_canceled();
            GovernanceProposals::<T>::insert(prop_id, prop);
            Self::deposit_event(Event::ProposalRejected(prop_id));
            Ok(())
        }

        /// Provides an AccountId for the pallet.
        /// This is used both as an origin check and deposit/withdrawal account.
        pub fn account_id() -> T::AccountId {
            PALLET_ID.into_account()
        }
    }

    pub trait WeightInfo {
        fn add_relayer() -> Weight;
        fn remove_relayer() -> Weight;
        fn vote_governance_proposal() -> Weight;
        fn new_governance_proposal() -> Weight;
        fn update_relayer_threshold() -> Weight;
        fn new_submission() -> Weight;
        fn close_governance_proposal() -> Weight;
    }

    /// For backwards compatibility and tests
    impl WeightInfo for () {
        fn add_relayer() -> Weight {
            Default::default()
        }

        fn remove_relayer() -> Weight {
            Default::default()
        }

        fn vote_governance_proposal() -> Weight {
            Default::default()
        }

        fn new_governance_proposal() -> Weight {
            Default::default()
        }

        fn update_relayer_threshold() -> Weight {
            Default::default()
        }

        fn new_submission() -> Weight {
            Default::default()
        }

        fn close_governance_proposal() -> Weight {
            Default::default()
        }
    }
}
