use crate::errors::Error;
use crate::traits::{AMTNode, BlockStore};
use cid::Cid;
use forest_encoding::de::{Deserialize};
use ipld_amt::{CollapsedNode, Link, Node as ForestNode};
use std::marker::PhantomData;
use crate::amt::nodes_for_height;

impl From<ipld_amt::Error> for Error {
    fn from(error: ipld_amt::Error) -> Self {
        let error_str = format!("forest_db error: {:?}", error);
        Error::Other(error_str)
    }
}

pub(crate) struct ForestAdaptedNode<V> {
    cid: Option<Cid>,
    inner: ForestNode<V>,
    _v: PhantomData<V>
}

impl <V> ForestAdaptedNode<V>{
    pub fn new(cid: Option<Cid>, inner: ForestNode<V>) -> Self {
        Self {
            cid,
            inner,
            _v: Default::default()
        }
    }
}

impl <V> AMTNode for ForestAdaptedNode<V> where V: for<'de>Deserialize<'de>{
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
                        let node = ForestAdaptedNode::new(
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

    fn get_by_cid<S: BlockStore>(&self, _cid: &Cid, _store: &S) -> Result<Option<Self>, Error> where Self: Sized {
        todo!()
    }

    fn cid(&self) -> Result<Cid, Error> {
        todo!()
    }
}
