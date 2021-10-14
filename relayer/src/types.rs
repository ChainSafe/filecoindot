// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin api types

use serde::{Deserialize, Serialize};

/// Response of the [`ChainHead`](https://docs.filecoin.io/reference/lotus-api/#chainhead) RPC call
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChainHead {
    pub jsonrpc: String,
    pub result: TipSet,
    pub id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TipSet {
    #[serde(rename = "Cids")]
    pub cids: Vec<Cid>,
    #[serde(rename = "Blocks")]
    pub blocks: Vec<Block>,
    #[serde(rename = "Height")]
    pub height: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "Miner")]
    pub miner: String,
    #[serde(rename = "Ticket")]
    pub ticket: ElectionProof,
    #[serde(rename = "ElectionProof")]
    pub election_proof: ElectionProof,
    #[serde(rename = "BeaconEntries")]
    pub beacon_entries: Vec<BeaconEntry>,
    #[serde(rename = "WinPoStProof")]
    pub win_po_st_proof: Vec<WinPoStProof>,
    #[serde(rename = "Parents")]
    pub parents: Vec<Cid>,
    #[serde(rename = "ParentWeight")]
    pub parent_weight: String,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "ParentStateRoot")]
    pub parent_state_root: Cid,
    #[serde(rename = "ParentMessageReceipts")]
    pub parent_message_receipts: Cid,
    #[serde(rename = "Messages")]
    pub messages: Cid,
    #[serde(rename = "BLSAggregate")]
    pub bls_aggregate: BlsAggregate,
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
    #[serde(rename = "BlockSig")]
    pub block_sig: BlsAggregate,
    #[serde(rename = "ForkSignaling")]
    pub fork_signaling: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeaconEntry {
    #[serde(rename = "Round")]
    pub round: i64,
    #[serde(rename = "Data")]
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlsAggregate {
    #[serde(rename = "Type")]
    pub bls_aggregate_type: i64,
    #[serde(rename = "Data")]
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElectionProof {
    #[serde(rename = "VRFProof")]
    pub vrf_proof: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cid {
    #[serde(rename = "/")]
    pub empty: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WinPoStProof {
    #[serde(rename = "RegisteredProof")]
    pub registered_proof: i64,
    #[serde(rename = "ProofBytes")]
    pub proof_bytes: String,
}
