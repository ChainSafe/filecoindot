// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::ocw::types::{Block, Cid, TipSet};

pub fn get_tip_set_by_height_1199840() -> TipSet {
    TipSet {
        cids: vec![
            Cid {
                inner: "bafy2bzacedbaliyx3k64d4rxy5q2og3wf5r5e2ra6bvf52ogldc6oad3jukbe"
                    .as_bytes()
                    .to_vec(),
            },
            Cid {
                inner: "bafy2bzaceblrey44c6ekyu7iu6dni4inrjgnyp7sgjrrgg3xnd3poxjrt2v2i"
                    .as_bytes()
                    .to_vec(),
            },
            Cid {
                inner: "bafy2bzacedd2wb4ijvvowm2gq3izffhl2oqlogigfubizozgbgo5l7rk73ick"
                    .as_bytes()
                    .to_vec(),
            },
            Cid {
                inner: "bafy2bzacecav4sjwonnjryjb5kmrint45yenyhorzn2it5noxdqhnudsquyoo"
                    .as_bytes()
                    .to_vec(),
            },
        ],
        blocks: vec![
            Block {
                messages: Cid {
                    inner: "bafy2bzacea4v4uwhlffeznk5sn2naw7zsl6tahsstc37ygongb3tppnqjosey"
                        .as_bytes()
                        .to_vec(),
                },
            },
            Block {
                messages: Cid {
                    inner: "bafy2bzacebyoxanjivzgsj3aisd5e5wrdai3oeqodxnkw2gbo2yctt33mbeeo"
                        .as_bytes()
                        .to_vec(),
                },
            },
            Block {
                messages: Cid {
                    inner: "bafy2bzacebizldd4vzjmjqp6gmmtixshxo7xlc4fl77drjmvhjg6zm5z6ojoc"
                        .as_bytes()
                        .to_vec(),
                },
            },
            Block {
                messages: Cid {
                    inner: "bafy2bzaceducpch7kljxpbsybi5uc3wljabh3zfbk2jvhpk56c2a4gnjbveoc"
                        .as_bytes()
                        .to_vec(),
                },
            },
        ],
        height: 1199840,
    }
}
