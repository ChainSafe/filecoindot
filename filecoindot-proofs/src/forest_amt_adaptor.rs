// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::amt::nodes_for_height;
use crate::errors::Error;
use crate::traits::{AMTNode, BlockStore, GetCid};
use cid::Cid;
use cid::Code::Blake2b256;
use forest_encoding::de::Deserialize;
use forest_encoding::to_vec;
use ipld_amt::{CollapsedNode, Link, Node as ForestNode};
use serde::de::Error as SerdeError;
use serde::{Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

const DEFAULT_BIT_WIDTH: usize = 3;

pub struct ForestAmtAdaptedNode<V> {
    cid: Option<Cid>,
    inner: ForestNode<V>,
    _v: PhantomData<V>,
}

impl<V> ForestAmtAdaptedNode<V> {
    pub fn new(cid: Option<Cid>, inner: ForestNode<V>) -> Self {
        Self {
            cid,
            inner,
            _v: Default::default(),
        }
    }
}

impl<V> Serialize for ForestAmtAdaptedNode<V>
where
    V: for<'de> Deserialize<'de> + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.inner).serialize(serializer)
    }
}

impl<'de, V> Deserialize<'de> for ForestAmtAdaptedNode<V>
where
    V: Serialize + for<'a> serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let node = Deserialize::deserialize(deserializer)?;
        let node: CollapsedNode<V> = node;
        let node = node
            .expand(DEFAULT_BIT_WIDTH)
            .map_err(|_| <D as serde::Deserializer<'de>>::Error::custom("cannot deserialized"))?;
        Ok(ForestAmtAdaptedNode::new(None, node))
    }
}

impl<V> GetCid for ForestAmtAdaptedNode<V>
where
    V: Serialize + for<'de> Deserialize<'de>,
{
    fn cid(&self) -> Result<Cid, Error> {
        match self.cid {
            Some(cid) => Ok(cid),
            None => {
                let bytes = to_vec(&self.inner)?;
                Ok(cid::new_from_cbor(&bytes, Blake2b256))
            }
        }
    }
}

impl<V> AMTNode for ForestAmtAdaptedNode<V>
where
    V: for<'de> Deserialize<'de> + Serialize,
{
    fn path_to_key<S: BlockStore>(
        &self,
        store: &S,
        bit_width: usize,
        height: usize,
        i: usize,
        path: &mut Vec<Vec<u8>>,
    ) -> Result<bool, Error> {
        let sub_i = i / nodes_for_height(bit_width, height);

        match &self.inner {
            ForestNode::Leaf { vals, .. } => {
                vals.get(i).ok_or(Error::NotFound)?;
                path.push(to_vec(self)?);
                Ok(true)
            }
            ForestNode::Link { links, .. } => {
                let link = links.get(sub_i).ok_or(Error::NotFound)?;

                match link {
                    Some(Link::Cid { cid, .. }) => {
                        let inner = store
                            .get::<CollapsedNode<V>>(cid)
                            .map_err(|_| Error::NotFound)?
                            .expand(bit_width)?;
                        let node = ForestAmtAdaptedNode::new(Some(*cid), inner);

                        node.path_to_key(
                            store,
                            bit_width,
                            height - 1,
                            i % nodes_for_height(bit_width, height),
                            path,
                        )
                    }
                    // We will not process dirty as we should have read
                    // directly from the FLUSHED storage.
                    Some(Link::Dirty(_)) => Err(Error::NotFound),
                    None => Err(Error::NotFound),
                }
            }
        }
    }

    fn get_by_cid<S: BlockStore>(
        &self,
        cid: &Cid,
        store: &S,
        bit_width: usize,
    ) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match &self.inner {
            // This is a leaf node, should not contain any cid
            ForestNode::Leaf { .. } => return Ok(None),
            ForestNode::Link { links, .. } => {
                for link in links {
                    match link {
                        Some(Link::Cid { cid: link_cid, .. }) => {
                            if link_cid != cid {
                                continue;
                            }
                            let inner = store
                                .get::<CollapsedNode<V>>(cid)
                                .map_err(|_| Error::NotFound)?
                                .expand(bit_width)?;
                            return Ok(Some(ForestAmtAdaptedNode::new(Some(*cid), inner)));
                        }
                        // We will not process dirty as we should have read
                        // directly from the FLUSHED storage.
                        Some(Link::Dirty(_)) => continue,
                        None => continue,
                    }
                }
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Amt, ForestAdaptedBlockStorage, ProofVerify, Verify};
    use ipld_amt::Amt as ForestAmt;
    use ipld_blockstore::MemoryDB;
    use serde_cbor::from_slice;

    #[test]
    fn test_basic_proof_generation() {
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
        let p = amt.generate_proof(0);
        assert_eq!(p.is_ok(), true);
    }

    #[test]
    fn test_verify_works() {
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
        let p = amt.generate_proof(0).unwrap();
        let raw_node = p.get(0).unwrap();
        let node: ForestAmtAdaptedNode<String> = from_slice(raw_node).unwrap();
        let r = ProofVerify::verify_proof::<ForestAmtAdaptedNode<String>>(
            p,
            node.cid().unwrap().to_bytes(),
        );
        assert_eq!(r.is_ok(), true);
    }
}
