use std::fs::File;

use crate::{
    block::{Datable, Hashable},
    chain::Chain,
    config::StoreConfig,
    metadata::SearchMetadata,
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
    pub fn new(config: &StoreConfig) -> Self {
        todo!("Impliment new store")
    }

    pub fn put(data: &impl Datable) -> Result<(), StoreError> {
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
