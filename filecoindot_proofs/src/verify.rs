use crate::errors::Error;
use cid::Cid;
use crate::traits::GetCid;
use serde_cbor::de::from_slice;

pub struct ProofVerify;

impl ProofVerify {
    /// Verify the proof and the the trie actually matches. Each cid in the proof
    /// is connected to its neighbours. The proof should match exactly in path from
    /// the root to the node.
    /// Note that proof[proof.len-1] == root_cid_bytes. This function does not assume
    /// the head of the proof to be equal to node_cid, as long as it's in the proof.
    pub fn verify_proof<N>(proof: Vec<Vec<u8>>, node_cid: &Cid) -> Result<(), Error> where N: GetCid + for<'de> serde::Deserialize<'de> {
        if proof.is_empty() {
            return Err(Error::VerificationFailed);
        }
        Self::traverse_and_match::<N>(&proof, proof.len() - 1, node_cid)
    }

    fn traverse_and_match<N>(
        proof: &[Vec<u8>],
        index: usize,
        target_cid: &Cid,
    ) -> Result<(), Error> where N: GetCid + for<'de> serde::Deserialize<'de> {
        let current_node: N = from_slice(&*proof[index]).map_err(|_| Error::VerificationFailed)?;
        if current_node.cid()? == *target_cid {
            return Ok(());
        }

        // We have not found the target_cid in the proof, search the next nodes.
        // The index is 0, we have reached the end of the proof, cannot proceed
        // any further, return error.
        if index == 0 {
            return Err(Error::VerificationFailed);
        }

        // now we search the previous index as we traverse deeper in to the trie
        Self::traverse_and_match::<N>(proof, index - 1, target_cid)
    }
}
