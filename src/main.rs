use clap::{arg, command};
use dirs::data_dir;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::{exit, Command};

mod hashing;

use hashing::hash_from_path;

struct Config {
    anno_dir: PathBuf,
    editor: String,
}

impl Config {
    fn from_env() -> Config {
        let default_anno_dir = data_dir().unwrap().join("anno");
        let anno_dir = env::var("ANNO_DIR").map_or(default_anno_dir, PathBuf::from);
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        fs::create_dir_all(&anno_dir).expect("Failed to create anno directory");

        Config { anno_dir, editor }
    }
}

fn read_to_string(path: &PathBuf) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn anno_read(config: &Config, path: &str) {
    let hash = hash_from_path(path);
    let anno_path = config.anno_dir.join(hash).with_extension("anno");

    match read_to_string(&anno_path) {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(e) => {
            eprintln!("Error reading anno file for `{}`: {}", path, e);
            exit(1);
        }
    }
}

fn anno_write(config: &Config, path: &str) {
    let hash = hash_from_path(path);
    let anno_path = config.anno_dir.join(hash).with_extension("anno");

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
        .get_matches();

    let config = Config::from_env();

    if let Some(file) = matches.get_one::<String>("read") {
        anno_read(&config, file);
    }

    if let Some(file) = matches.get_one::<String>("write") {
        anno_write(&config, file);
    }
}
