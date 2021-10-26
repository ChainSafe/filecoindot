use std::cmp::Ordering;
use std::marker::PhantomData;
use crate::traits::{HashedBits, Node};
use ipld_hamt::{Bitfield, BytesKey, Node as ForestNode, Pointer};
use cid::Cid;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use crate::errors::Error;

#[inline]
fn mkmask(n: u32) -> u32 {
    ((1u64 << n) - 1) as u32
}

type HashedKey = [u8; 32];

pub struct ForestAdaptedHashedBits {
    bit_width: u32,
    consumed: u32,
    b: HashedKey,
}

impl ForestAdaptedHashedBits {
    fn next_bits(&mut self, i: u32) -> u32 {
        let curbi = self.consumed / 8;
        let leftb = 8 - (self.consumed % 8);

        let curb = self.b[curbi as usize] as u32;
        match i.cmp(&leftb) {
            Ordering::Equal => {
                // bits to consume is equal to the bits remaining in the currently indexed byte
                let out = mkmask(i) & curb;
                self.consumed += i;
                out
            }
            Ordering::Less => {
                // Consuming less than the remaining bits in the current byte
                let a = curb & mkmask(leftb);
                let b = a & !mkmask(leftb - i);
                let c = b >> (leftb - i);
                self.consumed += i;
                c
            }
            Ordering::Greater => {
                // Consumes remaining bits and remaining bits from a recursive call
                let mut out = (mkmask(leftb) & curb) as u64;
                out <<= i - leftb;
                self.consumed += leftb;
                out += self.next_bits(i - leftb) as u64;
                out as u32
            }
        }
    }
}

impl HashedBits for ForestAdaptedHashedBits {
    type Value = u32;

    fn get(&self) -> Self::Value {
        let curbi = self.consumed / 8;
        let leftb = 8 - (self.consumed % 8);

        let curb = self.b[curbi as usize] as u32;
        match self.bit_width.cmp(&leftb) {
            Ordering::Equal => {
                // bits to consume is equal to the bits remaining in the currently indexed byte
                let out = mkmask(self.bit_width) & curb;
                out
            }
            Ordering::Less => {
                // Consuming less than the remaining bits in the current byte
                let a = curb & mkmask(leftb);
                let b = a & !mkmask(leftb - self.bit_width);
                let c = b >> (leftb - self.bit_width);
                c
            }
            Ordering::Greater => {
                // Consumes remaining bits and remaining bits from a recursive call
                mkmask(leftb) & curb
            }
        }
    }

    /// Returns next `i` bits of the hash and returns the value as an integer and returns
    /// Error when maximum depth is reached
    fn next(&mut self) -> Result<u32, Error> {
        let i = self.bit_width;
        if i > 8 {
            return Err(Error::InvalidHashBitLen);
        }
        if (self.consumed + i) as usize > self.b.len() * 8 {
            return Err(Error::MaxDepth);
        }
        Ok(self.next_bits(i))
    }
}

pub struct ForestAdaptedNode<K, V, H> {
    bitfield: Bitfield,
    pointers: Vec<Pointer<K, V, H>>,
}

impl<'de, K, V, H> Deserialize<'de> for ForestAdaptedNode<K, V, H>
    where
        K: DeserializeOwned,
        V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let (bitfield, pointers) = Deserialize::deserialize(deserializer)?;
        Ok(ForestAdaptedNode {
            bitfield,
            pointers,
        })
    }
}

impl<K: Eq, V, H> Node<K, V> for ForestAdaptedNode<K, V, H> {
    type GetHashBits = ForestAdaptedHashedBits;

    fn contains_key(&self, key: &K, hashed_bits: &Self::GetHashBits) -> bool {
        // self.inner
        todo!()
    }

    fn is_index(&self) -> bool {
        todo!()
    }

    fn cid(&self) -> Cid {
        todo!()
    }

    fn get_link_cid(&self, idx: <<Self as Node<K, V>>::GetHashBits as HashedBits>::Value) -> Option<Cid> {
        todo!()
    }
}
