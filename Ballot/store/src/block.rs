use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};

pub trait Hashable {
    fn sha256_hash(&self) -> Result<String>;
}

pub trait Datable {
    fn data(&mut self) -> Result<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block<H: Hashable, T: Hashable + Datable> {
    pub header: H,
    pub data: Option<T>,
}

impl Hashable for File {
    fn sha256_hash(&self) -> Result<String> {
        let reader = BufReader::new(self);
        let digest = sha256_digest(reader)?;
        Ok(HEXUPPER.encode(digest.as_ref()))
    }
}

impl Hashable for String {
    fn sha256_hash(&self) -> Result<String> {
        let reader = BufReader::new(self.as_bytes());
        let digest = sha256_digest(reader)?;
        Ok(HEXUPPER.encode(digest.as_ref()))
    }
}

impl Datable for File {
    fn data(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

impl Datable for String {
    fn data(&mut self) -> Result<String> {
        Ok(self.clone())
    }
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::io::Write;

    #[test]
    fn test_file_hashable() {
        let path = "file.txt";

        let mut output = File::create(path).unwrap();
        write!(output, "We will generate a digest of this text").unwrap();

        let input = File::open(path).unwrap();

        println!("{:?}", input.sha256_hash());
    }
}
