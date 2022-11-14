use sha256::try_digest;
use std::path::Path;

fn main() {
    let input = Path::new("/tmp/foo.bin");
    let val = try_digest(input).unwrap();
    println!("sha256: {}", val);
}
