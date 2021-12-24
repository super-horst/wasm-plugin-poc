use futures::future::Future;
use rust_embed::RustEmbed;
use std::{borrow::Cow, thread};

use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

#[derive(RustEmbed)]
#[folder = "../app"]
struct Asset;

fn request(req: Request<Body>) -> Response<Body> {
    let path = if req.uri().path() == "/" {
        // if there is no path, return default file
        "index.html"
    } else {
        // trim leading '/'
        &req.uri().path()[1..]
    };

    println!("requested {}", path);

    // query the file from embedded asset with specified path
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };

            let mut builder = Response::builder();

            if path.ends_with(".html") {
                builder.header("Content-Type", "text/html; charset=UTF-8");
            } else if path.ends_with(".js") {
                builder.header("Content-Type", "application/javascript; charset=UTF-8");
            } else if path.ends_with(".wasm") {
                builder.header("Content-Type", "application/wasm");
            }

            builder.status(200).body(body).unwrap()
        }
        None => Response::builder()
            .status(404)
            .body(Body::from("404 Not Found"))
            .unwrap(),
    }
}

fn main() {
    // listen on local ephemeral port
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(|| service_fn_ok(request));
    // query the port assigned by the os
    let port = server.local_addr().port();

    println!("http://127.0.0.1:{}", port);

    hyper::rt::run(server.map_err(|e| {
        eprintln!("server error: {}", e);
    }));
}
