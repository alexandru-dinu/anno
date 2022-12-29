use clap::{arg, command};
use lazy_static::lazy_static;
use serde_json::{Map, Value};
use std::fs::File;
use std::io::Read;
use std::process::{exit, Command};

mod hashing;

use hashing::hash_from_path;

struct Config {
    anno_dir: String,
    editor: String,
}

lazy_static! {
    static ref CONFIG: Config = {
        let cfg = parse_json("config.json");
        Config { // TODO: parse directly into struct?
            anno_dir: cfg.get("anno_dir").unwrap().as_str().unwrap().to_string(),
            editor: cfg.get("editor").unwrap().as_str().unwrap().to_string(),
        }
    };
}

fn parse_json(path: &str) -> Map<String, Value> {
    // TODO: return dict
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}: {}", e, path);
            exit(-1);
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let json: Value = serde_json::from_str(&contents).expect("Could not parse json");

    json.as_object().unwrap().clone()
}

fn read_path(path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", CONFIG.anno_dir, hash);

    let json = parse_json(&anno_path);

    for (key, value) in json {
        println!("{}: {}", key, value);
    }
}

fn write_path(path: &str) {
    let hash = hash_from_path(path);
    let anno_path = format!("{}/{}", CONFIG.anno_dir, hash);

    let status = Command::new(&CONFIG.editor)
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
            arg!(-r --read <FILE> "Read anno for given file")
                .required_unless_present("write")
                .conflicts_with("write"),
        )
        .arg(
            arg!(-w --write <FILE> "Write anno for given file")
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
