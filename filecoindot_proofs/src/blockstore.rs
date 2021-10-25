use crate::errors::Error;
use cid::Cid;
use forest_encoding::de::StdError;
use forest_encoding::from_slice;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use crate::node::Node;

/// Wrapper for database to handle inserting and retrieving ipld data with Cids
pub trait BlockStore<K, V, N: Node<K, V>> {
    /// Get typed object from block store by Cid.
    fn get<T>(&self, cid: &Cid) -> Result<Option<N>, Error>;
}
