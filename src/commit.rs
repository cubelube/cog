use std::fs;
use std::io::{prelude::*, Result};
use std::path::Path;

fn hash_object(data: &[u8]) -> String {
    use sha1::{Digest, Sha1};
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

pub fn write_commit(tree_oid: &str, message: &str) -> Result<String> {
    let data = format!("tree {}\n\n{}", tree_oid, message);
    write_object(data.as_bytes())
}