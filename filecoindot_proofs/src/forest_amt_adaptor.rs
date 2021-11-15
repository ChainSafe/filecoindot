use crate::errors::Error;
use crate::traits::{AMTNode, BlockStore};
use cid::Cid;
use cid::Code::Blake2b256;
use forest_encoding::de::{Deserialize};
use ipld_amt::{CollapsedNode, Link, Node as ForestNode};
use std::marker::PhantomData;
use forest_encoding::to_vec;
use serde::Serialize;
use crate::amt::nodes_for_height;

impl From<ipld_amt::Error> for Error {
    fn from(error: ipld_amt::Error) -> Self {
        let error_str = format!("forest_db error: {:?}", error);
        Error::Other(error_str)
    }
}

pub(crate) struct ForestAmtAdaptedNode<V> {
    cid: Option<Cid>,
    inner: ForestNode<V>,
    _v: PhantomData<V>
}

impl <V> ForestAmtAdaptedNode<V>{
    pub fn new(cid: Option<Cid>, inner: ForestNode<V>) -> Self {
        Self {
            cid,
            inner,
            _v: Default::default()
        }
    }
}

impl <V> AMTNode for ForestAmtAdaptedNode<V> where V: for<'de>Deserialize<'de> + Serialize {
    fn path_to_key<S: BlockStore>(&self, store: &S, bit_width: usize, height: usize, i: usize, path: &mut Vec<Vec<u8>>) -> Result<bool, Error> {
        let sub_i = i / nodes_for_height(bit_width, height);

        match &self.inner {
            ForestNode::Leaf { vals, .. } => {
                vals.get(i).ok_or(Error::NotFound)?;
                path.push(self.cid()?.to_bytes());
                Ok(true)
            },
            ForestNode::Link { links, .. } => {
                let link = links.get(sub_i)
                    .ok_or(Error::NotFound)?;

                match link {
                    Some(Link::Cid { cid, .. }) => {
                        let inner = store.get::<CollapsedNode<V>>(cid).map_err(|_| Error::NotFound)?
                            .expand(bit_width)?;
                        let node = ForestAmtAdaptedNode::new(
                            Some(cid.clone()),
                            inner
                        );

                        node.path_to_key(
                            store,
                            bit_width,
                            height - 1,
                            i % nodes_for_height(bit_width, height),
                            path
                        )
                    }
                    // We will not process dirty as we should have read
                    // directly from the FLUSHED storage.
                    Some(Link::Dirty(_)) => Err(Error::NotFound),
                    None => Err(Error::NotFound),
                }
            },
        }
    }

    fn get_by_cid<S: BlockStore>(&self, cid: &Cid, store: &S, bit_width: usize) -> Result<Option<Self>, Error> where Self: Sized {
        match &self.inner {
            // This is a leaf node, should not contain any cid
            ForestNode::Leaf { .. } => return Ok(None),
            ForestNode::Link { links, .. } => {
                for link in links {
                    match link {
                        Some(Link::Cid { cid: link_cid, .. }) => {
                            if link_cid != cid { continue; }
                            let inner = store.get::<CollapsedNode<V>>(cid).map_err(|_| Error::NotFound)?
                                .expand(bit_width)?;
                            return Ok(Some(ForestAmtAdaptedNode::new(
                                Some(cid.clone()),
                                inner
                            )));
                        }
                        // We will not process dirty as we should have read
                        // directly from the FLUSHED storage.
                        Some(Link::Dirty(_)) => continue,
                        None => continue,
                    }
                }
            },
        }
        Ok(None)
    }

    fn cid(&self) -> Result<Cid, Error> {
        let bytes = to_vec(&self.inner)?;
        Ok(cid::new_from_cbor(&bytes, Blake2b256))
    }
}
