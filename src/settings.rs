use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub steam_folder: String,
    pub target_folder: String,
    pub force_disable_update: bool,
}

// Note 01.11.2020:
// I'm starting to dislike this approach, as it is not the "Rust way".
// But it works, and i don't want to change it incase that i break something.
impl Settings {
    pub fn new() -> Settings {
        Settings { 
            steam_folder: "C:\\Program Files (x86)\\Steam\\userdata".to_string(),
            target_folder: "games".to_string(),
            force_disable_update: false,
        }
    }

    pub fn load(file: &Path) -> Settings{
        let mut f = match OpenOptions::new().read(true).write(false).open(&file) {
            Ok(file) => file,
            Err(error) => panic!("Error opening this file: {:?}", error)
        };

        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        toml::from_str(contents.as_str()).unwrap()
    }

    pub fn save(file: &Path, settings: &Settings) {
        let toml = toml::to_string_pretty(settings).unwrap();

        let mut f = match OpenOptions::new().read(true).write(true).create(true).open(&file) {
            Ok(file) => file,
            Err(error) => panic!("Error saving this file: {:?}", error)
        };
                 
        f.write_all(toml.as_bytes()).unwrap();
    }
}