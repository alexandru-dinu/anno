use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;

use crate::config;

pub fn hash_from_path(path: &str) -> String {
    // TODO: is it optimal to return a String here?
    let mut hasher = Sha256::new();

    let mut file = File::open(path).expect("Could not open file");
    let mut buffer = [0; config::READ_BUFFER_SIZE];

    loop {
        let count = file.read(&mut buffer).expect("Could not read from file");
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();

    let mut hash = String::new();
    for byte in result {
        hash.push_str(&format!("{:02x}", byte));
    }

    hash
}
