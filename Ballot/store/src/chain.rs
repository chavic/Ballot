use crate::block::*;
use crate::metadata::*;

pub struct Chain<I: ChainID, H: Hashable, T: Hashable> {
    pub chain_id: I,
    pub metadata: Metadata,
    pub blocks: Vec<Block<H, T>>,
}

pub trait ChainID {}

impl ChainID for String {}
