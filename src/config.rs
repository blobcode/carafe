// file that deals with all config parsing

use serde::Deserialize;
use std::fs;

// main config struct
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: i32,
    pub path: String,
}

pub fn read(path: String) -> Config {
    let file = fs::read_to_string("./carafe.toml").expect("Unable to read file");
    Config::from(toml::from_str(&file).unwrap())
}
