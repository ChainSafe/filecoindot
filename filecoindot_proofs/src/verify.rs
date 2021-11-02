use cid::Cid;
use crate::errors::Error;

pub fn verify(proof: Vec<Vec<u8>>, cid: &Cid) -> Result<(), Error> {
    let bytes = cid.to_bytes();
    match proof.iter().find(|p| **p == bytes) {
        Some(_) => Ok(()),
        None => Err(Error::VerificationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hamt::Hamt;
    use ipld_blockstore::MemoryDB;
    use ipld_hamt::Hamt as ForestHamt;
    use crate::{ForestAdaptedBlockStorage, ForestAdaptedHashAlgo, ForestAdaptedHashedBits, ForestAdaptedNode};

    #[test]
    fn test_basic_proof_generation() {
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

        let p = hamt.generate_proof(&1).unwrap();
        assert_eq!(verify(p.clone(), &cid).is_ok(), true);
        let not_exists = Cid::default();
        assert_eq!(verify(p, &not_exists).is_err(), true);
    }
}