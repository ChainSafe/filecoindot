// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use ipld_hamt::Hamt;

use cid::Code::Blake2b256;
use ipld_blockstore::{BSStats, BlockStore, TrackingBlockStore};

#[test]
fn test_basics() {
    let store = db::MemoryDB::default();
    let mut hamt = Hamt::<_, String, _>::new(&store);
    hamt.set(1, "world".to_string()).unwrap();

    assert_eq!(hamt.get(&1).unwrap(), Some(&"world".to_string()));
    hamt.set(1, "world2".to_string()).unwrap();
    assert_eq!(hamt.get(&1).unwrap(), Some(&"world2".to_string()));
}
