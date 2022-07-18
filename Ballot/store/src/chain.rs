use std::fs::File;

use crate::block::*;
use crate::metadata;
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
}
