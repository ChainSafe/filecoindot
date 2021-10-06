use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::Hash;

use crate::{Config, Error};

/// Possible votes a member can cast
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum VoteType {
    For,
    Against,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, Default)]
/// Info for keeping track of a motion being voted on.
/// Default is empty vectors for all votes
pub(crate) struct VoteAggregate<T: Config> {
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

impl<T: Config> VoteAggregate<T> {
    pub fn new(start_block: T::BlockNumber, end_block: T::BlockNumber) -> Self {
        VoteAggregate {
            for_votes: Vec::new(),
            against_votes: Vec::new(),
            start_block,
            end_block,
            is_executed: false,
            is_canceled: false,
        }
    }

    /// Derive the current proposal status.
    /// Need to pass in the current block number as the status is very time sensitive
    pub fn status(&self, now: &T::BlockNumber, threshold: u32) -> ProposalStatus {
        if self.is_canceled {
            return ProposalStatus::Canceled;
        }
        if self.is_executed {
            return ProposalStatus::Executed;
        }
        if self.is_active(now) {
            return ProposalStatus::Active;
        }

        // TODO: here the checking might not be sufficient
        // TODO: what if both num(YES) and num(NO) >= threshold? Are we saying threshold > total / 2?
        if self.for_votes.len() > threshold as usize {
            return ProposalStatus::Approved;
        }
        ProposalStatus::Rejected
    }

    /// Vote for the proposal. Will reject the operation is status is invalid
    pub fn vote(
        &mut self,
        who: T::AccountId,
        when: &T::BlockNumber,
        what: &VoteType,
    ) -> Result<(), Error<T>> {
        if self.is_voted(&who) {
            return Err(Error::<T>::AlreadyVoted);
        }

        if !self.is_active(when) {
            return Err(Error::<T>::ProposalInvalidStatus);
        }

        match what {
            VoteType::For => self.for_votes.push(who),
            VoteType::Against => self.against_votes.push(who),
        }

        Ok(())
    }

    /// Checks if the account has already voted
    fn is_voted(&self, who: &T::AccountId) -> bool {
        self.for_votes.contains(who) || self.against_votes.contains(who)
    }

    /// Whether the proposal is still active, i.e. can vote
    fn is_active(&self, now: &T::BlockNumber) -> bool {
        now.le(&self.end_block)
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
/// This represents an instance of a proposal that can be voted on.
/// It has been proposed and has an assigned nonce.
/// This extra abstraction is required since it may be desirable construct
/// multiple proposal instances out of a single proposal
pub(crate) struct GovernanceProposal<T: Config> {
    action: T::Action,
    issuer: T::AccountId,
    vote: VoteAggregate<T>,
}

impl<T: Config> GovernanceProposal<T> {
    pub fn new(
        action: T::Action,
        issuer: T::AccountId,
        start_block: T::BlockNumber,
        end_block: T::BlockNumber,
    ) -> Self {
        Self {
            action,
            issuer,
            vote: VoteAggregate::new(start_block, end_block),
        }
    }

    pub fn set_executed(&mut self) {
        self.vote.is_executed = true;
    }

    pub fn set_canceled(&mut self) {
        self.vote.is_canceled = true;
    }

    pub fn action(&self) -> T::Action {
        // TODO: is clone here recommended?
        self.action.clone()
    }

    pub fn hash(&self) -> <T as frame_system::Config>::Hash {
        T::Hashing::hash_of(self)
    }

    /// Derive the current proposal status.
    /// Need to pass in the current block number as the status is very time sensitive
    pub fn status(&self, now: &T::BlockNumber, threshold: u32) -> ProposalStatus {
        self.vote.status(now, threshold)
    }

    /// Vote for the proposal. Will reject the operation is status is invalid
    pub fn vote(
        &mut self,
        who: T::AccountId,
        when: &T::BlockNumber,
        what: &VoteType,
    ) -> Result<(), Error<T>> {
        self.vote.vote(who, when, what)
    }
}

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
    /// Proposal is executed
    Executed,
}
