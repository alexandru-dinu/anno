use clap::{arg, command};
use serde_json;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command};

// const ANNO_DIR: &str = concat!(env!("HOME"), "/.anno");
const ANNO_DIR: &str = "/tmp/anno";
const BUFFER_SIZE: usize = 4096;

fn hash_from_path(path: &str) -> String {
    // TODO: is it optimal to return a String here?
    let mut hasher = Sha256::new();

    let mut file = File::open(path).expect("Could not open file");
    let mut buffer = [0; BUFFER_SIZE];

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

fn read_path(path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", ANNO_DIR, hash);

    let mut file = match File::open(&anno_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}: {}", e, &anno_path);
            exit(-1);
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");
    let json: serde_json::Value = serde_json::from_str(&contents).expect("Could not parse json");
    for (key, value) in json.as_object().unwrap() {
        println!("{}: {}", key, value);
    }
}

fn write_path(path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", ANNO_DIR, hash);

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = Command::new(editor)
        .arg(anno_path)
        .status()
        .expect("Could not open editor");

    if !status.success() {
        eprintln!("Editor exited with non-zero status");
        exit(-1);
    }

    println!("ALL GOOD!");
}

fn main() {
    let matches = command!()
        .arg(
            arg!(-r --read <VALUE> "Read anno.")
                .required_unless_present("write")
                .conflicts_with("write"),
        )
        .arg(
            arg!(-w --write <VALUE> "Write anno.")
                .required_unless_present("read")
                .conflicts_with("read"),
        )
        .try_get_matches();

    if let Err(e) = matches {
        println!("{}", e);
        return;
    }

    let matches = matches.unwrap();

    if let Some(file) = matches.get_one::<String>("read") {
        read_path(file);
    }

    if let Some(file) = matches.get_one::<String>("write") {
        write_path(file);
    }
}
