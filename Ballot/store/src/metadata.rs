use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io::Result;
use std::io::Write;
use std::path::Path;

type JSON = Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    meta: JSON,
}

pub enum SearchMetadata {
    TimeRange,
}

impl Metadata {
    pub fn new(path: &Path) -> Result<Self> {
        let file_string = read_to_string(path)?;
        let metadata = serde_json::from_str(&file_string)?;
        Ok(metadata)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file_string = serde_json::to_string(self)?;
        let mut file = File::create(path)?;
        write!(file, "{}", file_string)?;
        Ok(())
    }
}
