use clap::{arg, command};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command};

mod config;
mod hashing;

use hashing::hash_from_path;

fn read_path(path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", config::ANNO_DIR, hash);

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
    let anno_path = format!("{}/{}", config::ANNO_DIR, hash);

    let status = Command::new(config::get_editor())
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
