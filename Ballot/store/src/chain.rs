use std::fs::File;

use crate::block::*;
use crate::metadata::*;

pub struct Chain<H: Hashable, T: Hashable + Datable> {
    pub metadata: Metadata,
    blocks: Vec<Block<H, T>>,
}

impl Chain<String, File> {
    pub fn new(metadata: Metadata) -> Chain<String, File> {
        todo!("Impliment new chain")
    }

    pub fn build(start: Block<String, File>, metadata: Metadata) -> Chain<String, File> {
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

    pub fn next_header(&self) -> Result<String> {
        todo!("Impliment next hash")
    }

    pub fn prev_header(&self) -> Result<String> {
        todo!("Impliment next hash")
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}
