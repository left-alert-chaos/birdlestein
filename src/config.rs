//config.rs - store config data and save data with toml
use serde::Deserialize;
use std::fs;
use toml;

//doesn't do much rn, but in future will hold most if not all config data for runtime reference
#[derive(Deserialize, Default, Debug)]
pub struct Settings {
    pub(crate) last_project: String,
    pub(crate) projects: std::collections::HashMap<String, Project>,
}

//holds all info about a project, which isn't much rn.
#[derive(Deserialize, Default, Debug)]
pub struct Project {
    pub(crate) path: String,
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
                Settings {
                    ..Default::default()
                }
            }
        } else {
            //if it couldn't be processed, default settings
            Settings {
                ..Default::default()
            }
        }
    }
}
