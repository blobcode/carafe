use env_logger::Env;
use log::info;
use path_slash::PathExt;

mod args;
mod config;
mod server;

fn main() {
    // setup logging
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    // load args
    let args = args::parse().unwrap(); // never fails since all values are optional

    // load config
    let config = config::init(args);

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

    // start server
    server::run(config);
}
