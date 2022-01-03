// manages webserver and request handling

use ascii::AsciiString;
use log::info;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::Server;

use crate::config::Config;

pub fn run(config: Config) {
    // start server
    let server = Server::http(format!("0.0.0.0:{}", config.port)).unwrap();

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break,
        };

        info!("{:?}", rq);

        // init string
        let url = rq.url().to_string();

        // create filepath
        let pathstring = format!("{}{}", &config.root.to_str().unwrap(), &url);

        // create pathbuf
        let mut path = PathBuf::from(&pathstring);

        // index.html checking
        if path.is_dir() {
            let index = Path::new("index.html");
            path = path.join(index);
        }

        // read file
        let file = fs::File::open(&path);

        // validate file
        if file.is_ok() {
            let response = tiny_http::Response::from_file(file.unwrap());

            let response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                // infer filetype
                // unwrap() is ok since values have been verified
                value: AsciiString::from_ascii(
                    infer::get_from_path(&path).unwrap().unwrap().mime_type(),
                )
                .unwrap(),
            });

            let _ = rq.respond(response);
        } else {
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            let _ = rq.respond(rep);
        }
    }
}
