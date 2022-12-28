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

mod tests {
    #[test]
    fn test_hash_from_path() {
        let files = vec!["/usr/bin/ls", "/usr/bin/vi"];

        for file in files {
            let src_hash = super::hash_from_path(file);
            let tgt_hash = std::process::Command::new("sha256sum")
                .arg(file)
                .output()
                .expect("Could not run sha256sum")
                .stdout
                .split(|&x| x == b' ')
                .next()
                .unwrap()
                .to_vec();

            assert_eq!(src_hash, String::from_utf8(tgt_hash).unwrap());
        }
    }
}
