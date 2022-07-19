use std::fs::File;

use crate::block::*;
use crate::metadata::*;

pub struct Chain<H: Hashable, T: Hashable + Datable> {
    pub metadata: Metadata,
    pub blocks: Vec<Block<H, T>>,
}

impl Chain<String, File> {
    pub fn build_new(start: Block<String, File>, metadata: Metadata) -> Chain<String, File> {
        todo!("Impliment block")
    }

    pub fn build_seg(metadata: Metadata, start: usize, end: usize) -> Chain<String, File> {
        todo!("Imblicment block segment")
    }

    pub fn add_block(&self, block: Block<String, File>) {
        todo!("add block")
    }

    pub fn find_block_with_hash_seg(
        &self,
        hash_seg: String,
        search: SearchMetadata,
    ) -> Result<Option<Block<String, File>>> {
        todo!("Find block with has")
    }

    pub fn find_hash_with_seg(
        &self,
        hash_seg: String,
        block: Option<Block<String, File>>,
    ) -> Result<Option<String>> {
        todo!("hash seg")
    }
}
