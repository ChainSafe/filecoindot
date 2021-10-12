use crate::types::ProposalStatus;
use crate::{
    tests::mock::*, BlockSubmissionProposals, Error, MessageRootCidCounter, VerifiedBlocks,
};
use frame_support::{assert_err, assert_ok};

#[test]
fn submit_block_vote_works() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).unwrap(),
            1
        );

        let p = BlockSubmissionProposals::<Test>::get(&block_cid).unwrap();
        assert_eq!(*p.get_status(), ProposalStatus::Active);
    });
}

#[test]
fn submit_block_vote_fails_not_relayer() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(ALICE),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::NotRelayer
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),
            true
        );
    });
}

#[test]
fn submit_block_vote_fails_already_voted() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER1),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::AlreadyVoted
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).unwrap(),
            1
        );
    });
}

#[test]
fn submit_block_vote_resolve_rejected() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        System::set_block_number(100);
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER2),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::ProposalExpired
        );
        assert_eq!(VerifiedBlocks::<Test>::get(&block_cid), false);
        assert_eq!(
            BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),
            true
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),
            true
        );
    });
}

#[test]
fn submit_block_vote_resolve_approved() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        // assert_eq!(*p.get_status(), ProposalStatus::Approved);
        assert_eq!(VerifiedBlocks::<Test>::get(&block_cid), true);
        // assert_eq!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(), true);
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),
            true
        );
    });
}

#[test]
fn submit_block_vote_resolve_completed() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        System::set_block_number(100);
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER4),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::ProposalCompleted
        );
        assert_eq!(VerifiedBlocks::<Test>::get(&block_cid), false);
        assert_eq!(
            BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),
            true
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),
            true
        );
    });
}
