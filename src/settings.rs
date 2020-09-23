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

impl Settings {
    pub fn new() -> Settings {
        Settings { 
            steam_folder: "C:\\Program Files (x86)\\Steam\\userdata".to_string(),
            target_folder: "games".to_string(),
            force_disable_update: false,
        }
    }

    pub fn load(file: &Path) -> Settings{
        let f = OpenOptions::new().read(true).write(false).open(&file);

        //return Err(io::Error::new(io::ErrorKind::NotFound, "The file 'appids.json' was not found!"));

        let mut f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening this file: {:?}", error)
        };

        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        //let _settings: Settings = toml::from_str(contents.as_str()).unwrap();

        toml::from_str(contents.as_str()).unwrap()
    }

    pub fn save(file: &Path, settings: &Settings) {
        let toml = toml::to_string_pretty(settings).unwrap();

        let mut f = OpenOptions::new().read(true).write(true).create(true).open(&file).unwrap();
                 
        f.write_all(toml.as_bytes()).unwrap();
    }
}