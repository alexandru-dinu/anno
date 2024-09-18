use clap::{arg, command};
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::process::{exit, Command};

mod hashing;

use hashing::hash_from_path;

#[derive(Deserialize)]
struct Config {
    anno_dir: String,
    editor: String,
}

impl Config {
    fn parse_from_file(path: &str) -> Result<Self, serde_json::Error> {
        let contents = read_to_string(path).map_err(serde_json::Error::io)?;
        serde_json::from_str(&contents)
    }
}

fn read_to_string(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn anno_read(config: &Config, path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", config.anno_dir, hash);
    match read_to_string(&anno_path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn anno_write(config: &Config, path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", config.anno_dir, hash);

    let status = Command::new(&config.editor)
        .arg(anno_path)
        .status()
        .expect("Could not open editor");

    if !status.success() {
        eprintln!("Editor exited with non-zero status");
        exit(1);
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!(-r --read <FILE> "Read anno for given file")
                .required_unless_present("write")
                .conflicts_with("write"),
        )
        .arg(
            arg!(-w --write <FILE> "Write anno for given file")
                .required_unless_present("read")
                .conflicts_with("read"),
        )
        .try_get_matches()
        .expect("Error parsing arguments");

    let config = Config::parse_from_file("config.json").expect("Failed to load config");

    if let Some(file) = matches.get_one::<String>("read") {
        anno_read(&config, file);
    }

    if let Some(file) = matches.get_one::<String>("write") {
        anno_write(&config, file);
    }
}
