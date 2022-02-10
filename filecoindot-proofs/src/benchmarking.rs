// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{
    deserialize_to_node, Amt, ForestAdaptedBlockStorage, ForestAdaptedHashAlgo,
    ForestAdaptedHashedBits, ForestAdaptedNode, ForestAmtAdaptedNode, GetCid, HAMTNodeType, Hamt,
};
use ipld_amt::Amt as ForestAmt;
use ipld_blockstore::MemoryDB;
use ipld_hamt::Hamt as ForestHamt;
use serde_cbor::from_slice;

#[allow(clippy::type_complexity)]
pub fn hamt_proof_generation() -> (Vec<Vec<u8>>, Vec<u8>) {
    let bs = MemoryDB::default();
    let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

    let max = 1000;
    for i in 1..max {
        fhamt.set(i, i.to_string()).unwrap();
    }

    let cid = fhamt.flush().unwrap();
    let store = ForestAdaptedBlockStorage::new(bs);
    let hamt: Hamt<
        ForestAdaptedBlockStorage<MemoryDB>,
        usize,
        String,
        ForestAdaptedHashedBits,
        ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
        ForestAdaptedHashAlgo,
    > = Hamt::new(&cid, &store, 8).unwrap();
    let mut p = hamt.generate_proof(&(max / 2)).unwrap();
    p.reverse();
    let raw_node = p.get(0).unwrap();
    let node: HAMTNodeType = deserialize_to_node(None, raw_node).unwrap();
    (p, node.cid().unwrap().to_bytes())
}

pub fn amt_proof_generation(n: usize) -> (Vec<Vec<u8>>, Vec<u8>) {
    let bs = MemoryDB::default();
    let mut famt = ForestAmt::new(&bs);

    let max = 1000;
    for i in 1..max {
        famt.set(i, i.to_string()).unwrap();
    }

    let cid = famt.flush().unwrap();
    let store = ForestAdaptedBlockStorage::new(bs);
    let amt: Amt<ForestAdaptedBlockStorage<MemoryDB>, ForestAmtAdaptedNode<String>> =
        Amt::load(&cid, &store).unwrap();
    let p = amt.generate_proof(n).unwrap();
    let raw_node = p.get(0).unwrap();
    let node: ForestAmtAdaptedNode<String> = from_slice(raw_node).unwrap();
    (p, node.cid().unwrap().to_bytes())
}
