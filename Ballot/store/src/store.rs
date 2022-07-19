use std::fs::File;
use std::io::Write;

use crate::{
    block::{Block, Datable, Hashable},
    chain::Chain,
    config::StoreConfig,
    metadata::{self, Metadata, SearchMetadata},
    StoreError,
};

pub struct ImutableStore {
    pub config: StoreConfig,
    pub chain: Chain<String, File>,
}

pub struct LinkedStore {
    pub config: StoreConfig,
    pub chain: Chain<String, File>,
}

impl ImutableStore {
    pub fn new(config: StoreConfig) -> Result<Self, StoreError> {
        let mut chain_metadata = config.path.clone();
        chain_metadata.push("metadata.b");

        let metadata = Metadata::new(&chain_metadata)?;
        let chain = Chain::new(metadata);
        Ok(ImutableStore { config, chain })
    }

    pub fn put(&self, data: &mut (impl Datable + Hashable)) -> Result<(), StoreError> {
        let prev_header = {
            if self.chain.is_empty() {
                String::from("0").sha256_hash().unwrap()
            } else {
                self.chain.prev_header().unwrap()
            }
        };

        let next_header = { data.sha256_hash().unwrap() };

        let mut block_path = self.config.data_path.clone();
        block_path.push(format!("/{}.bd", next_header));

        let mut file = File::create(block_path)?;
        let data = data.data().unwrap();
        write!(file, "{}", data)?;

        let block: Block<String, File> = Block {
            prev_header,
            data: Some(file),
        };

        self.chain.add_block(block);

        Ok(())
    }

    pub fn find(
        &self,
        data: &(impl Datable + Hashable),
        search_metadata: &SearchMetadata,
    ) -> Result<Option<()>, StoreError> {
        todo!("Impliment find data")
    }

    pub fn count(&self) -> Result<i32, StoreError> {
        todo!("Impliment count")
    }
}

impl LinkedStore {
    pub fn new(config: &StoreConfig) -> Self {
        todo!("Impliment new linked store")
    }

    pub fn put(data: &impl Datable, linked_data: &impl Datable) -> Result<(), StoreError> {
        todo!("Impliment put store data")
    }

    pub fn find(
        data: &impl Datable,
        search_metadata: &SearchMetadata,
    ) -> Result<Option<()>, StoreError> {
        todo!("Impliment find data")
    }

    pub fn count() -> Result<i32, StoreError> {
        todo!("Impliment count")
    }
}
