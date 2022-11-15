use blake3;
use sha256::try_digest;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Path::new(&args[1]);

    let mut now = Instant::now();
    let val = try_digest(input).unwrap();
    let mut took: f64 = (now.elapsed().as_millis() as f64) / 1000.0;
    println!("sha256: {} ({})", val, took);

    now = Instant::now();
    let data: Vec<u8> = fs::read(input).unwrap();
    took = (now.elapsed().as_millis() as f64) / 1000.0;
    println!("read: {}", took);

    now = Instant::now();
    let val = blake3::hash(&data);
    took = (now.elapsed().as_millis() as f64) / 1000.0;
    println!("blake3: {} ({})", val, took);
}
