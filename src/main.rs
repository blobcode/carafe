use env_logger::Env;
use log::info;

mod config;
mod server;

fn main() {
    /*
    todo:
    cli arguments (main.rs)
    better config file finding (config.rs)
    multiple directory support (config.rs)
    */

    // setup logging
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let config = config::read("./carafe.toml".to_string());

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
    info!("serving {} on port {}", config.path, config.port);
    server::run(config.port, config.path);
}
