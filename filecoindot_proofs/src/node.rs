use crate::traits::{BitMap, Node};
use cid::Cid;
use std::cell::{RefCell, RefMut};

#[derive(Debug)]
pub struct KeyValuePair<K: Eq, V>(K, V);

impl<K: Eq, V> KeyValuePair<K, V> {
    pub fn key(&self) -> &K {
        &self.0
    }
}

impl<K: Eq, V> KeyValuePair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValuePair(key, value)
    }
}

/// Node in Hamt tree which contains bitfield of set indexes and pointers to nodes
#[derive(Debug)]
pub enum Pointer<K: Eq, V> {
    KeyValue(Vec<KeyValuePair<K, V>>),
    Link(Cid),
}

pub(crate) struct NodeInner<K: Eq, V, B: BitMap> {
    cid: Cid,
    bitmap: RefCell<B>,
    pointers: Vec<Pointer<K, V>>,
}

impl<K, V, B> Node<K, V, B> for NodeInner<K, V, B>
where
    K: Eq,
    B: BitMap,
{
    fn bitmap(&self) -> RefMut<B> {
        self.bitmap.borrow_mut()
    }

    fn get_pointer(&self, idx: usize) -> Option<&Pointer<K, V>> {
        self.pointers.get(idx)
    }

    fn cid(&self) -> Cid {
        self.cid.clone()
    }
}
