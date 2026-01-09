//config.rs - store config data and save data with toml
use toml::{self, ser::Error};
use serde::{Serialize, Deserialize};
use std::{fs};
use crate::files;

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    pub(crate) value: String,
}

impl Settings {
    //create a settings obj from a config file
    pub fn from_file(file_path: &str) -> Self {
        //if the file is read successfully
        if let Ok(path) = fs::read_to_string(file_path) {
            //if the toml is processed successfully
            if let Ok(me) = toml::from_str(&path) {
                me
            } else {
                //if it couldn't be processed, default settings
                Settings {..Default::default()}
            }
        } else {
            //if it couldn't be processed, default settings
            Settings {..Default::default()}
        }
    }

    pub fn freeze(&self) {
        let frozen: Result<String, Error> = toml::to_string(self);
        if let Ok(text) = frozen {
            files::write_file(&String::from("birdlestein.toml"), text);
        }
    }
}