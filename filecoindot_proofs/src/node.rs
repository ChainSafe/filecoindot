use crate::traits::{BitMap, HashedBits, Node};
use cid::Cid;
use std::marker::PhantomData;

/// Node in Hamt tree which contains bitfield of set indexes and pointers to nodes
#[derive(Debug)]
enum NodeType<'a, K, V, B, N>
where
    K: Eq,
    N: Node<K, V>,
    B: BitMap<Index = <<N as Node<K, V>>::GetHashBits as HashedBits>::Value>,
{
    KeyValue {
        key: K,
        val: V,
    },
    Index {
        bitmap: B,
        /// Link nodes pointing to other nodes
        links: Vec<Option<Box<&'a N>>>,
    },
}

pub(crate) struct NodeInner<
    'a,
    K: Eq,
    V,
    B: BitMap<Index = <<N as Node<K, V>>::GetHashBits as HashedBits>::Value>,
    H: HashedBits,
    N: Node<K, V, GetHashBits = H>,
> {
    cid: Cid,
    bit_width: u8,
    node_type: NodeType<'a, K, V, B, N>
}

impl<'a, K, V, B, H, N> Node<K, V> for NodeInner<'a, K, V, B, H, N>
where
    K: Eq,
    B: BitMap<Index = <<N as Node<K, V>>::GetHashBits as HashedBits>::Value>,
    H: HashedBits,
    N: Node<K, V, GetHashBits = H>,
{
    type GetHashBits = H;

    fn contains_key(&self, k: &K, hash_bits: &Self::GetHashBits) -> bool {
        match &self.node_type {
            NodeType::KeyValue { key, .. } => key == k,
            NodeType::Index { bitmap, .. } => {
                let idx = hash_bits.get();
                bitmap.is_bit_set(idx)
            }
        }
    }

    fn is_index(&self) -> bool {
        match &self.node_type {
            NodeType::Index { .. } => true,
            _ => false
        }
    }

    fn cid(&self) -> Cid {
        self.cid.clone()
    }

    fn get_link_cid(
        &self,
        idx: <<Self as Node<K, V>>::GetHashBits as HashedBits>::Value,
    ) -> Option<Cid> {
        match &self.node_type {
            NodeType::Index { bitmap, links} => {
                let i = bitmap.count_ones(idx);
                links.get(i).unwrap().as_ref().map(|n| n.cid())
            },
            _ => None,
        }
    }
}
