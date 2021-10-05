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
    use frame_system::pallet_prelude::*;

    use crate::types::{ProposalDetail, ProposalStatus, QuorumStrategy};

    type ProposalId<T> = <T as frame_system::Config>::Hash;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Origin used to administer the pallet
        /// TODO: Do we need dynamically change admin here?
        type ManagerOrigin: EnsureOrigin<Self::Origin>;
        /// The lifetime of a proposal
        type ProposalLifetime: Get<Self::BlockNumber>;
        /// The weight for this pallet's extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Relayers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, OptionQuery>;

    #[pallet::storage]
    pub(crate) type Proposals<T: Config> =
    StorageMap<_, Blake2_128Concat, ProposalId<T>, ProposalDetail<T>, OptionQuery>;

    #[pallet::storage]
    pub(super) type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub(super) type RelayerThreshold<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub(super) type VotingPeriod<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Relayer added to set
        RelayerAdded(T::AccountId),
        /// Relayer removed from set
        RelayerRemoved(T::AccountId),
        /// Relayer threshold updated to value
        RelayerThresholdUpdate(u32),
        /// Vote for the proposal casted
        VoteForCasted(ProposalId<T>, T::AccountId),
        /// Vote against the proposal casted
        VoteAgainstCasted(ProposalId<T>, T::AccountId),
        /// Proposal created
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
        /// Relayer has already voted
        AlreadyVoted,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// TODO: fix all the weights later on
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
            RelayerThreshold::<T>::set(threshold);

            Self::deposit_event(Event::RelayerThresholdUpdate(threshold));
            Ok(())
        }

        // ************** Proposal Lifecycle *************
        /// TODO: who can create proposals?
        #[pallet::weight(T::WeightInfo::new_proposal())]
        pub fn new_proposal(origin: OriginFor<T>, block_cid: Vec<u8>, message_root_cid: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // ensure!(!Self::is_relayer(&who), Error::<T>::MustBeRelayer);

            let start_block = <frame_system::Pallet<T>>::block_number();
            let end_block = VotingPeriod::<T>::get();
            let proposal: ProposalDetail<T> = ProposalDetail::new(who, block_cid, message_root_cid, start_block, end_block);

            let proposal_id: ProposalId<T> = proposal.hash();
            if Proposals::<T>::contains_key(proposal_id) {
                return Err(Error::<T>::ProposalAlreadyExists.into());
            }
            Proposals::<T>::insert(proposal_id, proposal);
            Self::deposit_event(Event::ProposalCreated(proposal_id));

            Ok(())
        }

        // **************** Voting Related ***************
        // TODO： QUESTION - How does the lifecycle of proposal work again?

        /// Commits a vote in favour of the provided proposal.
        #[pallet::weight(T::WeightInfo::vote_for_proposal())]
        pub fn vote_for_proposal(origin: OriginFor<T>, proposal_id: ProposalId<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Self::is_relayer(&who), Error::<T>::MustBeRelayer);

            Proposals::<T>::try_mutate(proposal_id, |proposal| {
                let proposal = proposal.as_mut().ok_or(Error::<T>::RelayerAlreadyExists)?;

                let now = <frame_system::Pallet<T>>::block_number();
                proposal.vote_for(who.clone(), &now)?;
                Self::deposit_event(Event::VoteForCasted(proposal_id, who));
                Ok(())
            })
        }

        /// Commits a vote in favour of the provided proposal.
        #[pallet::weight(T::WeightInfo::vote_against_proposal())]
        pub fn vote_against_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Self::is_relayer(&who), Error::<T>::MustBeRelayer);

            Proposals::<T>::try_mutate(proposal_id, |proposal| {
                let proposal = proposal.as_mut().ok_or(Error::<T>::RelayerAlreadyExists)?;

                let now = <frame_system::Pallet<T>>::block_number();
                proposal.vote_against(who.clone(), &now)?;
                Self::deposit_event(Event::VoteAgainstCasted(proposal_id, who));
                Ok(())
            })
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

            <Relayers<T>>::insert(&relayer, true);
            RelayerCount::<T>::mutate(|i| *i += 1);

            Self::deposit_event(Event::RelayerAdded(relayer));
            Ok(())
        }

        /// Removes a relayer from the set
        /// Caller ensure the invoker has appropriate admin roles
        fn unregister_relayer(relayer: T::AccountId) -> DispatchResult {
            ensure!(Self::is_relayer(&relayer), Error::<T>::RelayerInvalid);

            Relayers::<T>::remove(&relayer);
            RelayerCount::<T>::mutate(|i| *i -= 1);

            Self::deposit_event(Event::RelayerRemoved(relayer));
            Ok(())
        }

        /// Checks if who is a relayer
        fn is_relayer(who: &T::AccountId) -> bool {
            Relayers::<T>::get(who).unwrap_or(false)
        }

        // ============== Voting Related =============
        /// TODO: who is calling this function?
        fn try_resolve_proposal(
            prop_id: ProposalId<T>,
            prop: &mut ProposalDetail<T>,
        ) -> DispatchResult {
            let now = <frame_system::Pallet<T>>::block_number();

            let quorum_strategy = QuorumStrategy::Simple {
                threshold: RelayerThreshold::<T>::get() as usize,
                total: RelayerCount::<T>::get() as usize,
            };

            match prop.try_resolve(&quorum_strategy, now)? {
                ProposalStatus::Approved => Self::finalize_execution(prop_id, prop),
                ProposalStatus::Rejected => Self::cancel_execution(prop_id, prop),
                _ => Ok(()),
            }
        }

        fn finalize_execution(
            _prop_id: ProposalId<T>,
            _prop: &mut ProposalDetail<T>,
        ) -> DispatchResult {
            Ok(())
        }

        fn cancel_execution(_prop_id: ProposalId<T>, _prop: &mut ProposalDetail<T>) -> DispatchResult {
            Ok(())
        }
    }

    pub trait WeightInfo {
        fn add_relayer() -> Weight;
        fn remove_relayer() -> Weight;
        fn vote_against_proposal() -> Weight;
        fn vote_for_proposal() -> Weight;
        fn new_proposal() -> Weight;
        fn update_relayer_threshold() -> Weight;
    }

    /// For backwards compatibility and tests
    impl WeightInfo for () {
        fn add_relayer() -> Weight {
            Default::default()
        }

        fn remove_relayer() -> Weight {
            Default::default()
        }

        fn vote_against_proposal() -> Weight {
            Default::default()
        }

        fn vote_for_proposal() -> Weight {
            Default::default()
        }

        fn new_proposal() -> Weight {
            Default::default()
        }

        fn update_relayer_threshold() -> Weight {
            Default::default()
        }
    }
}
