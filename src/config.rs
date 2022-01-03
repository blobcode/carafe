// file that deals with all config parsing

use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

// main config struct
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u32,
    pub root: PathBuf,
}

// defaults to fall back to
impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8080,
            root: PathBuf::from_str("./").unwrap(),
        }
    }
}

pub fn read(path: PathBuf) -> Config {
    let file = fs::read_to_string(path).expect("Unable to read file");
    Config::from(toml::from_str(&file).unwrap())
}

pub fn default() -> Config {
    Config::default()
}
