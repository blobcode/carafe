use env_logger::Env;
use log::{error, info};
use path_slash::PathExt;
use std::{path::PathBuf, str::FromStr};

mod args;
mod config;
mod server;

fn main() {
    // setup logging
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let args = args::parse().unwrap(); // never fails since all values are optional

    // set to defaults
    let mut config = config::default();

    // check if file in cwd is a thing
    if PathBuf::from_str("./carafe.toml").unwrap().is_file() {
        config = config::read(PathBuf::from_str("./carafe.toml").unwrap()) // never fails
    }

    // checks if args are a thing
    if args.port.is_some() {
        config.port = args.port.unwrap();
    };

    if args.dir.is_some() {
        config.root = args.dir.unwrap();
    };

    // validates file provided with -c
    match args.configpath {
        Some(path) => {
            if path.is_file() {
                let root = path.parent().unwrap().to_path_buf();
                config = config::read(path);
                config.root = root.join(config.root);
            } else {
                error!("provided config file is invalid")
            }
        }
        _ => {}
    };

    // logos look cool
    println!(
        r#"
                      __     
                     / _|    
  ___ __ _ _ __ __ _| |_ ___ 
 / __/ _` | '__/ _` |  _/ _ \
| (_| (_| | | | (_| | ||  __/
 \___\__,_|_|  \__,_|_| \___|
    "#
    );

    // log startup info
    info!(
        "serving {} on port {}",
        // format text
        config.root.to_slash().unwrap(),
        config.port
    );
    server::run(config);
}
