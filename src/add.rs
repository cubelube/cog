use std::fs;
use std::io::{prelude::*, Result};
use std::path::Path;
use sha1::{Digest, Sha1};

fn hash_object(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn write_object(data: &[u8]) -> Result<String> {
    let oid = hash_object(data);
    let object_path = Path::new(".cog/objects").join(&oid);
    let mut file = fs::File::create(object_path)?;
    file.write_all(data)?;
    Ok(oid)
}

pub fn add_file(filename: &str) -> Result<String> {
    let data = fs::read(filename)?;
    write_object(&data)
}