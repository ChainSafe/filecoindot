use crate::{Config, Error};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::Hash;

/// The status of the proposal
/// Expected status transition:
///     Active -> Approved -> Executed
///               Rejected    Canceled
///                           Expired
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) enum ProposalStatus {
    /// The proposal is active and relayers can start voting
    Active,
    /// Proposal is approved
    Approved,
    /// Proposal is rejected
    Rejected,
    /// Proposal is cancelled
    Canceled,
    /// Proposal is expired
    Expired,
    /// Proposal is executed
    Executed,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug)]
pub(crate) struct ProposalDetail<T: Config> {
    /// The account that started this proposal
    proposer: T::AccountId,
    /// The cid of the filecoin block.
    block_cid: Vec<u8>,
    /// The message root cid
    message_root_cid: Vec<u8>,
    // TODO: how many relayers will there be again? Save some bits?
    /// The accounts with `for` votes
    for_votes: Vec<T::AccountId>,
    /// The accounts with `against` votes
    against_votes: Vec<T::AccountId>,
    /// The block number that the proposal started
    start_block: T::BlockNumber,
    /// The block number that the proposal ended
    end_block: T::BlockNumber,
    /// Whether the proposal is executed
    is_executed: bool,
    /// Whether the proposal is canceled
    is_canceled: bool,
}

impl<T: Config> ProposalDetail<T> {
    pub fn new(
        proposer: T::AccountId,
        block_cid: Vec<u8>,
        message_root_cid: Vec<u8>,
        start_block: T::BlockNumber,
        end_block: T::BlockNumber,
    ) -> Self {
        ProposalDetail {
            proposer,
            block_cid,
            message_root_cid,
            for_votes: Vec::new(),
            against_votes: Vec::new(),
            start_block,
            end_block,
            is_executed: false,
            is_canceled: false,
        }
    }

    pub fn hash(&self) -> <T as frame_system::Config>::Hash {
        // TODO: maybe remove clone here?
        T::Hashing::hash_of(&(self.block_cid.clone(), self.message_root_cid.clone()))
    }

    /// Derive the current proposal status.
    /// Need to pass in the current block number as the status is very time sensitive
    pub fn status(&self, _now: &T::BlockNumber) -> ProposalStatus {
        // TODO: the logic here needs to be clarified
        todo!()
    }

    /// Vote for the proposal. Will reject the operation is status is invalid
    pub fn vote_for(&mut self, who: T::AccountId, now: &T::BlockNumber) -> Result<(), Error<T>> {
        if self.is_voted(&who) {
            return Err(Error::<T>::AlreadyVoted);
        }

        let status = self.status(now);
        if status != ProposalStatus::Active {
            return Err(Error::<T>::ProposalInvalidStatus);
        }

        self.for_votes.push(who);
        Ok(())
    }

    /// Vote against the proposal. Will reject the operation is status is invalid
    pub fn vote_against(
        &mut self,
        who: T::AccountId,
        now: &T::BlockNumber,
    ) -> Result<(), Error<T>> {
        if self.is_voted(&who) {
            return Err(Error::<T>::AlreadyVoted);
        }

        let status = self.status(now);
        if status != ProposalStatus::Active {
            return Err(Error::<T>::ProposalInvalidStatus);
        }

        self.against_votes.push(who);
        Ok(())
    }

    fn is_voted(&self, who: &T::AccountId) -> bool {
        self.for_votes.contains(&who) || self.against_votes.contains(&who)
    }

    pub fn try_resolve(
        &mut self,
        strategy: &QuorumStrategy,
        now: T::BlockNumber,
    ) -> Result<ProposalStatus, Error<T>> {
        let status = self.status(&now);

        // TODO: we are not handling Canceled?
        // TODO: this is also very strange as the status can be Approved first then Rejected again?
        // TODO: should we consider once not active then not allowing any operations?
        ensure!(
            status != ProposalStatus::Executed,
            Error::<T>::ProposalAlreadyExecuted
        );
        ensure!(
            status != ProposalStatus::Expired,
            Error::<T>::ProposalExpired
        );

        if !strategy.formed(self.against_votes.len(), self.for_votes.len()) {
            return Ok(ProposalStatus::Rejected);
        }

        if self.for_votes.len() >= self.against_votes.len() as usize {
            return Ok(ProposalStatus::Approved);
        }

        Ok(ProposalStatus::Rejected)
    }
}

pub(crate) enum QuorumStrategy {
    Simple { total: usize, threshold: usize },
}

impl QuorumStrategy {
    pub fn formed(&self, against_votes: usize, for_votes: usize) -> bool {
        match self {
            QuorumStrategy::Simple { total, threshold } => {
                let quorum = total / 2;
                let min = threshold.max(&quorum);
                against_votes + for_votes > *min
            }
        }
    }
}
