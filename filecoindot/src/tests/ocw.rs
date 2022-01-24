// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{
    ocw::{
        api::{Req, Resp},
        offchain_worker,
        types::TipSet,
    },
    tests::mock::*,
    Relayers,
};
use sp_core::{
    offchain::{testing, OffchainDbExt, OffchainWorkerExt, TransactionPoolExt},
    Decode,
};
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{offchain::storage::StorageValueRef, RuntimeAppPublic};
use std::sync::Arc;

const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";
const FILECOIN_API: &str = "http://filecoin.api";
const CHAIN_HEAD_RESP: &[u8] = br#"
{
  "result": {
    "Cids": [
      { "/": "bafy2bzaced43kofq4s4fvsv7esoh2tlst56wngbszkhawfgey4geszwsjj3ww" },
      { "/": "bafy2bzacebbpmhfnuuxwofaaabwuhhr3zpmuypuy6yxtan4cs4cs4zbtiymzo" },
      { "/": "bafy2bzacecuxwhxpehke5u2e677ies6fmtywzgnzoagjanlwtpkkkeofspysa" },
      { "/": "bafy2bzaced2mght4hso3osblkjjsvcsqby7h3sg3vgq4n4f5fzh5b3dxvftua" },
      { "/": "bafy2bzacedngsxfjlk7mbqtpp2i3nzpjoyp4iim4ha4ozrxc57dr7vfm3ten2" }
    ],
    "Blocks": [
      {
        "Messages": {
          "/": "bafy2bzacedhiusftmig7alne5gkuywadhrnnketndnjygr7gdpw4w4cq2u5b2"
        }
      },
      {
        "Messages": {
          "/": "bafy2bzacecrt4qybm3klbdri5ofk4lpvnafmo2jc2brpaq25n6pjcr2k3jmcq"
        }
      },
      {
        "Messages": {
          "/": "bafy2bzacedjurwnsndbb4zz7kk7lxtrs7g4dxsasjlmggvouhfikug543dnta"
        }
      },
      {
        "Messages": {
          "/": "bafy2bzaceba54ejfcbd2cvzqbvfaczbtdot2tfy7l4hlmnn64vxm5fn5bolog"
        }
      },
      {
        "Messages": {
          "/": "bafy2bzacebqjfvo2k2x6holp6olhpj7f5wuy35p4mnfzkpqm7agqolpf7qmtg"
        }
      }
    ],
    "Height": 1273769
  }
}
"#;

#[test]
fn should_submit_vote_in_ocw() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let (pool, pool_state) = testing::TestTransactionPoolExt::new();

    // set keystore
    let keystore = KeyStore::new();

    // set up relayer
    let relayer =
        SyncCryptoStore::sr25519_generate_new(&keystore, crate::crypto::Public::ID, Some(PHRASE))
            .unwrap();

    // set expected response
    {
        let mut state = state.write();
        let params: Vec<()> = Default::default();
        state.expect_request(testing::PendingRequest {
            method: "POST".into(),
            uri: FILECOIN_API.into(),
            response: Some(CHAIN_HEAD_RESP.to_vec()),
            sent: true,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: serde_json::to_string(&Req {
                id: 0,
                method: "Filecoin.ChainHead",
                jsonrpc: "2.0",
                params,
            })
            .unwrap()
            .as_bytes()
            .to_vec(),
            ..Default::default()
        });
    }

    // register extensions
    let mut t = ExtBuilder::default().build();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));
    t.register_extension(TransactionPoolExt::new(pool));
    t.register_extension(KeystoreExt(Arc::new(keystore)));

    // execute in test env
    t.execute_with(|| {
        // add inserted key as relayer
        Relayers::<Test>::insert(&relayer, ());

        // set rpc endpoint
        let rpc = StorageValueRef::persistent("FILECOIN_RPC".as_bytes());
        rpc.set(&vec![FILECOIN_API.as_bytes().to_vec()]);

        // bootstrap ocw on block 1
        offchain_worker::<Test>(1u32.into()).unwrap();

        // get submited transactions
        let resp = serde_json::from_slice::<Resp<TipSet>>(CHAIN_HEAD_RESP)
            .unwrap()
            .result;
        for (i, raw_tx) in pool_state.read().transactions.iter().enumerate() {
            let tx = Extrinsic::decode(&mut &*raw_tx.to_owned()).unwrap();
            assert_eq!(tx.signature, Some((i as u64, ())));
            assert_eq!(
                tx.call,
                Call::FileCoinModule(crate::Call::submit_block_vote {
                    block_cid: resp.cids[i].inner.to_vec(),
                    message_root_cid: resp.blocks[i].messages.inner.to_vec()
                })
            );
        }
    });
}
