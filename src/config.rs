// file that deals with all config parsing

use log::error;
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

use crate::args::AppArgs;

// main config struct
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u32,
    pub root: PathBuf,
    pub serve: Option<Vec<RouteConfig>>,
}

// defaults to fall back to
impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8080,
            root: PathBuf::from_str("./").unwrap(),
            serve: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub route: String,
    pub path: PathBuf,
}

pub fn init(args: AppArgs) -> Config {
    // set to defaults
    let mut config = default();

    // check if file in cwd is a thing
    if PathBuf::from_str("./carafe.toml").unwrap().is_file() {
        config = read(PathBuf::from_str("./carafe.toml").unwrap()) // never fails
    }

    // checks if args are a thing
    if args.port.is_some() {
        config.port = args.port.unwrap();
    };

    if args.dir.is_some() {
        config.root = args.dir.unwrap();
    };

    // validates file provided with -c
    if let Some(path) = args.configpath {
        if path.is_file() {
            let root = path.parent().unwrap().to_path_buf();
            config = read(path);
            config.root = root.join(config.root);
        } else {
            error!("provided config file is invalid")
        }
    };

    config
}

pub fn read(path: PathBuf) -> Config {
    let file = fs::read_to_string(path).expect("Unable to read file");

    toml::from_str(&file).unwrap()
}

pub fn default() -> Config {
    Config::default()
}
