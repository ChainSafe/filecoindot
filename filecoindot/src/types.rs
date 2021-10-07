use frame_support::pallet_prelude::*;

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
}

impl<T: Config> VoteAggregate<T> {
    pub fn new(start_block: T::BlockNumber, end_block: T::BlockNumber) -> Self {
        VoteAggregate {
            for_votes: Vec::new(),
            against_votes: Vec::new(),
            start_block,
            end_block,
        }
    }

    /// Derive the current proposal status.
    /// Need to pass in the current block number as the status is very time sensitive
    pub fn status(&self, now: &T::BlockNumber, threshold: u32) -> ProposalStatus {
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

/// The filecoin block submission proposal
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) struct BlockSubmissionProposal<T: Config> {
    proposer: T::AccountId,
    message_root: Vec<u8>,
    vote: VoteAggregate<T>,
}

impl<T: Config> BlockSubmissionProposal<T> {
    pub fn new(
        proposer: T::AccountId,
        message_root: Vec<u8>,
        start_block: T::BlockNumber,
        end_block: T::BlockNumber,
    ) -> Self {
        Self {
            proposer,
            message_root,
            vote: VoteAggregate::new(start_block, end_block),
        }
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
}
