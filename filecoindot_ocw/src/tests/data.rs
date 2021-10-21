// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    api::ChainGetTipSetByHeightResult,
    types::{BeaconEntry, Block, BlsAggregate, Cid, ElectionProof, WinPoStProof},
};

pub fn get_tip_set_by_height_1199840() -> ChainGetTipSetByHeightResult {
    ChainGetTipSetByHeightResult {
        cids: vec![
            Cid {
                empty: "bafy2bzacedbaliyx3k64d4rxy5q2og3wf5r5e2ra6bvf52ogldc6oad3jukbe".to_string(),
            },
            Cid {
                empty: "bafy2bzaceblrey44c6ekyu7iu6dni4inrjgnyp7sgjrrgg3xnd3poxjrt2v2i".to_string(),
            },
            Cid {
                empty: "bafy2bzacedd2wb4ijvvowm2gq3izffhl2oqlogigfubizozgbgo5l7rk73ick".to_string(),
            },
            Cid {
                empty: "bafy2bzacecav4sjwonnjryjb5kmrint45yenyhorzn2it5noxdqhnudsquyoo".to_string(),
            },
        ],
        blocks: vec![
            Block {
                miner: "f0733242".to_string(),
                ticket: ElectionProof {
                    vrf_proof: "hgNW658F5ud+QBZ6ED7v5gT7I8aVhdKes8fSRJrJaRNdxdUp4gz+3dk7f13BA+WTDxPLozhZtlk97cnW/NO7szY0zM4DxVwBDOHWLjx7OYzzfYXj56icybIKI+PF1S1t".to_string(),
                },
                election_proof: ElectionProof {
                    vrf_proof: "qanIItIDpw16T+dWXIyPcB94/TJD36zpTwQAAXrhfM3SqYtswfJ8KjTZgWlU7GSQAtxK44AuSZvg1GeTh0EdhL9x0o3RueUFucNPoggirwLyVJfB6jq/EN5uLyHEdDjY".to_string(),
                },
                beacon_entries: vec![
                    BeaconEntry {
                        round: 1295684,
                        data: "pCNFJKhlTrL+7Gp0YtqUjOaWP0YX3J76euYOZZqkwYYynvVC3VJVSCXW0MLm+XO5GdesTAjsUoxdMy6myma5LWeXqggCU6bMxwtS5Kz3+v0+UjvsN1zbJWwG/imJAJp5".to_string(),
                    },
                ],
                win_po_st_proof: vec![
                    WinPoStProof {
                        po_st_proof: 4,
                        proof_bytes: "qYV9RCTinHmlrEh2T0vEnwfwCJJWD+T4ZmoE10s7FjMlkqwnyfMbbAvh/cKG5s90ijY1AyoPW1C5VeZBzzECFnUuY7oN/gE6zSc6+RuheFi5VXTAHRx5SouAI9qlocQgFvQaxMlDDtOs1CtHh3vHo5v5iXf2SX9ZjHO5pz+ZXb+eV0pbNH/Neo/xK5FPH1EYlNO64rvECMGnQvaR1brWqXTO3Ce2QjXGp8W8esm/h6Q0IM2gyn4G8SkSyGpqOarl".to_string(),
                    },
                ],
                parents: vec![
                    Cid {
                        empty: "bafy2bzaceaue3sd5qijwafs7ycyhpcdzb4kpribce7zx2ervpjsqutdvl7vxq".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzaceabf3mewrf6xha3als5misrzb6erhkeenphfbl2ylodfx4dzngh5u".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacecw372lcniqqo3je4tjobgsbq6bsqhkv4rmmkstr46vlxqn63meem".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacebfoleuxh5ebggsi272s5c7xff64kdslkfcoe5km6ere73fc42yns".to_string(),
                    },
                ],
                parent_weight: "27397340404".to_string(),
                height: 1199840,
                parent_state_root: Cid {
                    empty: "bafy2bzacedldmgjc3pjvrhvsgg4tx736dmvhw2nuwekx2xolrbos4cgh6clza".to_string(),
                },
                parent_message_receipts: Cid {
                    empty: "bafy2bzaceb75fgw6yg7ekoqhw5ai3nuodtnnuerhcbamnkbhhtzkago6vacjg".to_string(),
                },
                messages: Cid {
                    empty: "bafy2bzacea4v4uwhlffeznk5sn2naw7zsl6tahsstc37ygongb3tppnqjosey".to_string(),
                },
                bls_aggregate: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "jV3n9VBzIGlcpVuklDaQ3Z3aKkHXoq6bnEdH9IVDshU2jIjc3xu+NCKyQkFhPdOuGalL/ud9MNkm/D++6zf1sHerLPXJ0TDGi0J5cELIzyLOQfqFVbJhz1rvAaBxdija".to_string(),
                },
                timestamp: 1634301600,
                block_sig: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "hC8thFZuAB8fsHMHhEA1Y6/LeypZUuoH+r3RMfbyNvFkIeRaGFg7+Ks2Z65RSmUHACdGYEj49NNWUZcMVTBTwX823Grm+zwW5qiB/XKG2ZH9XCv6TfDy2fMbjKcnZoG3".to_string(),
                },
                fork_signaling: 0,
            },
            Block {
                miner: "f01038199".to_string(),
                ticket: ElectionProof {
                    vrf_proof: "hREYIhTXRlr8lSE8Sp5iC7EMRLULop+8+vbAKb99RqEsTZjfgTenBWvYb8IjRc1ZFoF/Gxvr6LTDVRa5skSeWa7LILZ1kf4sWWfa8l0irLveVSgn6QB/PSSQ32D5HQpC".to_string(),
                },
                election_proof: ElectionProof {
                    vrf_proof: "rsxBCMqKoRiSlV//tyvSKOAOnAT3MO6HPXcJe2HVOepsEQ+VVJpSmw6o8MpaVq/tCubf6zvZ2eMB/0JkMPrA30FSco0QDI70IeRr6QE9EELBhBLDzhZGBcGm8me8e2Ad".to_string(),
                },
                beacon_entries: vec![
                    BeaconEntry {
                        round: 1295684,
                        data: "pCNFJKhlTrL+7Gp0YtqUjOaWP0YX3J76euYOZZqkwYYynvVC3VJVSCXW0MLm+XO5GdesTAjsUoxdMy6myma5LWeXqggCU6bMxwtS5Kz3+v0+UjvsN1zbJWwG/imJAJp5".to_string(),
                    },
                ],
                win_po_st_proof: vec![
                    WinPoStProof {
                        po_st_proof: 3,
                        proof_bytes: "jjDdF126FO5XUbk0SdXCEQdUTbrtOpDXyNpnBdqkB/1a0bkoHmeAKc8r4RXdRL6uiPHrGE5ImEBvR9v6PFkn/BvmCtLLcrH2dB3bu96gLEBklPFLNlCoWdMF80L3ZPEQBo+oRAIjnODJdTQL4wCtDPg8rWFGRXIkRdse1M03Som1mk+a8H0FWCQopqcTwqkBg7f9KRxKK4yPM/cVC1AehGIWVUdkjMktHynciU5S7ClqXoUadbxuQZFkrIfzW1j1".to_string(),
                    },
                ],
                parents: vec![
                    Cid {
                        empty: "bafy2bzaceaue3sd5qijwafs7ycyhpcdzb4kpribce7zx2ervpjsqutdvl7vxq".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzaceabf3mewrf6xha3als5misrzb6erhkeenphfbl2ylodfx4dzngh5u".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacecw372lcniqqo3je4tjobgsbq6bsqhkv4rmmkstr46vlxqn63meem".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacebfoleuxh5ebggsi272s5c7xff64kdslkfcoe5km6ere73fc42yns".to_string(),
                    },
                ],
                parent_weight: "27397340404".to_string(),
                height: 1199840,
                parent_state_root: Cid {
                    empty: "bafy2bzacedldmgjc3pjvrhvsgg4tx736dmvhw2nuwekx2xolrbos4cgh6clza".to_string(),
                },
                parent_message_receipts: Cid {
                    empty: "bafy2bzaceb75fgw6yg7ekoqhw5ai3nuodtnnuerhcbamnkbhhtzkago6vacjg".to_string(),
                },
                messages: Cid {
                    empty: "bafy2bzacebyoxanjivzgsj3aisd5e5wrdai3oeqodxnkw2gbo2yctt33mbeeo".to_string(),
                },
                bls_aggregate: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "phYYbetc5nFCn0I5Y3tNi9PX7Y5bvNtAHGnkdtOrPWSsHNxJqwsTpLW7rfAhGZrSA+WiGB52hG340A52gE05j2xucucMX4H4YDiqXTubSAfrzdzLrL8l4uBOE4zBmVxI".to_string(),
                },
                timestamp: 1634301600,
                block_sig: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "izTyZTkQRgtBx2TcbrjEjoX849Cxr8c65uBeoqqs5SQuEX6jyNKFR2GXN3hAku3kByOvJ6s+7kiEqHIKRSw6jaMWOA5YfwzBWNKxGtNHdfaSTtQtH44n6+UfTWypvRfY".to_string(),
                },
                fork_signaling: 0,
            },
            Block {
                miner: "f0442370".to_string(),
                ticket: ElectionProof {
                    vrf_proof: "uXYqEqAU8JuL39NLpklBh0gQCLbU4bj4ShPVcwxbZ8BXB0NxRBUQGy/Dhy+B17yYCfGOb9x66NC9d8gJYZfenx61Ktd1g0/gVg4qn3+8JGKe49sy9mQLyLQSUkHWJT0I".to_string(),
                },
                election_proof: ElectionProof {
                    vrf_proof: "i7kHYkfDyQFcMTBftRsrwNnt4a8JBEykpiAr49HHOZpJrk/C2x5rxBI2+FsdH8tzARYLI9qWS+L77lKThahv64jFQSQIrsUlTXX49C+v9COK9w3eTKu22LK43ILxXws7".to_string(),
                },
                beacon_entries: vec![
                    BeaconEntry {
                        round: 1295684,
                        data: "pCNFJKhlTrL+7Gp0YtqUjOaWP0YX3J76euYOZZqkwYYynvVC3VJVSCXW0MLm+XO5GdesTAjsUoxdMy6myma5LWeXqggCU6bMxwtS5Kz3+v0+UjvsN1zbJWwG/imJAJp5".to_string(),
                    },
                ],
                win_po_st_proof: vec![
                    WinPoStProof {
                        po_st_proof: 4,
                        proof_bytes: "kl2OrN5GNG/zhfT+qqI3dIeSqvqu03mCSm1IUp8Xcg+eSmstj7I61a8Fe/sRL4gmtycK2J08inrYJJmRGXDZjrS03wEvTTnDquDttvDSiW8aJC1EkpqmqDxxZp9TgRdNC9D5+Afq5Je7GDA9YEoL7y4J/H8ZGMJNZhmj402dHw+gVaIwor4lBOvh4IhAPqC+piREOH+efoXusbMIvNjlV0deDfcG09xQQ/2J8Lksa7VYVbMu4cgV1DtyQFieclTp".to_string(),
                    },
                ],
                parents: vec![
                    Cid {
                        empty: "bafy2bzaceaue3sd5qijwafs7ycyhpcdzb4kpribce7zx2ervpjsqutdvl7vxq".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzaceabf3mewrf6xha3als5misrzb6erhkeenphfbl2ylodfx4dzngh5u".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacecw372lcniqqo3je4tjobgsbq6bsqhkv4rmmkstr46vlxqn63meem".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacebfoleuxh5ebggsi272s5c7xff64kdslkfcoe5km6ere73fc42yns".to_string(),
                    },
                ],
                parent_weight: "27397340404".to_string(),
                height: 1199840,
                parent_state_root: Cid {
                    empty: "bafy2bzacedldmgjc3pjvrhvsgg4tx736dmvhw2nuwekx2xolrbos4cgh6clza".to_string(),
                },
                parent_message_receipts: Cid {
                    empty: "bafy2bzaceb75fgw6yg7ekoqhw5ai3nuodtnnuerhcbamnkbhhtzkago6vacjg".to_string(),
                },
                messages: Cid {
                    empty: "bafy2bzacebizldd4vzjmjqp6gmmtixshxo7xlc4fl77drjmvhjg6zm5z6ojoc".to_string(),
                },
                bls_aggregate: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "gnFyHlLZAwN2PTfiyNXQ0ZkA5LSl+nzmblojPyvIM/0zC5glLpHbX3tJaTbCmHlxGdsBS4J8GFcNru7s6paawg+6An18UImd/+KA9do0URrAaSMMH9VcIneaVKHOPclr".to_string(),
                },
                timestamp: 1634301600,
                block_sig: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "jZhkpnlJlBWQIbMZEnfDnllcw+/K5ndCp2h4kSMU/m47sZvSF1ff7hfpeGEryp/OBWxQiD4rwDMP40/vT6xj3k5YQl4r0+sR1PhuKXG97QA3GaNmmwkSAW7wlK0k4hDV".to_string(),
                },
                fork_signaling: 0,
            },
            Block {
                miner: "f0133501".to_string(),
                ticket: ElectionProof {
                    vrf_proof: "rP1n2/a02PX4ObrZlrA20xf9z9jSN+DFfnziyIhLRsotSSbR2SaNLsWjE50jG2dpDqDWYcNVVvVEyh8r0w53fJCcMb6Zq95gyu8nePcjtxm+ZtN24UbOPnYwGnlslENp".to_string(),
                },
                election_proof: ElectionProof {
                    vrf_proof: "ufVJNHy1K9J7A6zyGvO1SK81FOCIt0z6s9Ip5lil8AcpZd9mDeHuM5fUjptl69j9BJGPsi7w4JjfhQ12mPZiCLiHH+Pt76jPvu0f+6ZfcQ3ec2dYQjK9rvvySXJpHGYb".to_string(),
                },
                beacon_entries: vec![
                    BeaconEntry {
                        round: 1295684,
                        data: "pCNFJKhlTrL+7Gp0YtqUjOaWP0YX3J76euYOZZqkwYYynvVC3VJVSCXW0MLm+XO5GdesTAjsUoxdMy6myma5LWeXqggCU6bMxwtS5Kz3+v0+UjvsN1zbJWwG/imJAJp5".to_string(),
                    },
                ],
                win_po_st_proof: vec![
                    WinPoStProof {
                        po_st_proof: 4,
                        proof_bytes: "hShOJhfo/A3SfHXtG1j5CuC7q/8rDAYoVyb6r2a5Js3LDfLRfmDlO47kyFSsfhwvrEymOUfn/8J+ct8GKkQu2ZqpHQQUYXf00cIKitKiv8XXZ7u3eMRQYSEDLMXs8NKyC3kb4KvzDL/I7yFmPkkOyuiu6J3yVRWXb3kfSsNVM5EcZcUWhrfCm1OezMTYPChQuMS/rQmPU3BQLcs9uFsx4ue5SO5kf/w0oP5VzZgFs/f+jKEq5+tTMTP4OIAh+BMJ".to_string(),
                    },
                ],
                parents: vec![
                    Cid {
                        empty: "bafy2bzaceaue3sd5qijwafs7ycyhpcdzb4kpribce7zx2ervpjsqutdvl7vxq".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzaceabf3mewrf6xha3als5misrzb6erhkeenphfbl2ylodfx4dzngh5u".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacecw372lcniqqo3je4tjobgsbq6bsqhkv4rmmkstr46vlxqn63meem".to_string(),
                    },
                    Cid {
                        empty: "bafy2bzacebfoleuxh5ebggsi272s5c7xff64kdslkfcoe5km6ere73fc42yns".to_string(),
                    },
                ],
                parent_weight: "27397340404".to_string(),
                height: 1199840,
                parent_state_root: Cid {
                    empty: "bafy2bzacedldmgjc3pjvrhvsgg4tx736dmvhw2nuwekx2xolrbos4cgh6clza".to_string(),
                },
                parent_message_receipts: Cid {
                    empty: "bafy2bzaceb75fgw6yg7ekoqhw5ai3nuodtnnuerhcbamnkbhhtzkago6vacjg".to_string(),
                },
                messages: Cid {
                    empty: "bafy2bzaceducpch7kljxpbsybi5uc3wljabh3zfbk2jvhpk56c2a4gnjbveoc".to_string(),
                },
                bls_aggregate: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "sjROXiMW6wPu4T3vRsfeJAK8ybd0pVgJ5IyM/bd9B3C+x9K46WIzT+fMjRRsRxKtEb7tL/myXgcco8+hutNru/h66wdC3ohJMKqjDMbArIp4olrabG8pjbTH8yNBwHm8".to_string(),
                },
                timestamp: 1634301600,
                block_sig: BlsAggregate {
                    bls_aggregate_type: 2,
                    data: "h+tWWo3z2LDfrohvbC+5WQdqcWAcaA67P4JApx7G2Dl7C2e+WlV1q6BRo67GBU6BCmpEufcj5yfc9Tlewrpp2Ql2oKuDiforqDCMzDbX8qMTzql2SR1KLP3DQO7zGmsq".to_string(),
                },
                fork_signaling: 0,
            },
        ],
        height: 1199840,
    }
}
