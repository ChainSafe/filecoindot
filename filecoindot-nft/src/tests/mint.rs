use crate::tests::mock::*;
use frame_support::assert_ok;

#[test]
fn mint_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FilecoinNFT::mint(Origin::signed(ALICE), vec![], vec![]));
    });
}

#[test]
fn tokens_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FilecoinNFT::mint(Origin::signed(ALICE), vec![], vec![]));
        assert_ok!(FilecoinNFT::mint(Origin::signed(ALICE), vec![2], vec![]));
        FilecoinNFT::tokens(&ALICE);
    });
}

#[test]
fn tokens_details_works() {
    ExtBuilder::default().build().execute_with(|| {
        let cid = vec![1u8];
        let proof = vec![vec![2u8]];
        assert_ok!(FilecoinNFT::mint(
            Origin::signed(ALICE),
            cid.clone(),
            proof.clone()
        ));
        assert_ok!(FilecoinNFT::mint(Origin::signed(ALICE), vec![2], vec![]));
        let r = FilecoinNFT::token_detail(0).unwrap();

        assert_eq!(r.cid, cid);
        assert_eq!(r.proof, proof);
    });
}
