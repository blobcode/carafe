// manages webserver and request handling

use ascii::AsciiString;
use log::info;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::Server;

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

pub fn run(port: i32, path: PathBuf) {
    // start server
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break,
        };

        info!("{:?}", rq);

        let url = rq.url().to_string();
        let pathstring = format!("{}{}", &path.to_str().unwrap(), &url);
        let mut path = PathBuf::from(&pathstring);

        if path.is_dir() {
            let index = Path::new("index.html");
            path = path.join(index);
        }

        let file = fs::File::open(&path);

        if file.is_ok() {
            let response = tiny_http::Response::from_file(file.unwrap());

            let response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(get_content_type(&path)).unwrap(),
            });

            let _ = rq.respond(response);
        } else {
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            let _ = rq.respond(rep);
        }
    }
}
