use crate::errors::Error;

pub(crate) trait BitMap {
    /// The max number of bits this BitMap can hold
    fn size(&self) -> usize;
    /// Clear the bit at the specified index, required index < size()
    fn clear_bit(&mut self, index: usize) -> Result<(), Error>;
    /// Performs the and operation between two BitMaps
    fn and(&self, rhs: Self) -> Self;
}

pub(crate) struct U128BitMap {
    num: u128
}

impl Default for U128BitMap {
    fn default() -> Self {
        U128BitMap { num: 0 }
    }
}

impl BitMap for U128BitMap {
    fn size(&self) -> usize {
        todo!()
    }

    fn clear_bit(&mut self, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn and(&self, rhs: Self) -> Self {
        todo!()
    }
}