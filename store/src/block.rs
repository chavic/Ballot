use serde::{Deserialize, Serialize};

pub trait Hashable {}
pub trait Datable {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block<H: Hashable, T: Hashable> {
    pub block_header: H,
    pub data: Option<T>,
    pub link_header: Option<H>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHash {}
impl Hashable for BlockHash {}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData<T: Datable> {
    data: Vec<T>,
}
impl Hashable for BlockData<String> {}

impl Block<BlockHash, BlockData<String>> {}

impl Datable for String {}
