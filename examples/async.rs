extern crate futures;
extern crate reqwest;
extern crate tokio;

use std::mem;
use std::io::{self, Cursor};
use futures::{Future, Stream};
use reqwest::async::{ClientBuilder, Decoder};


fn fetch() -> impl Future<Item=(), Error=()> {
    let client_builder = ClientBuilder::new();
    let client = client_builder
        .danger_accept_invalid_certs(true)
        .http_version(reqwest::HttpVersion::Http11)
        .use_rustls_tls()
        .build()
        .unwrap();
    client
    // .get("https://duckduckgo.com")
        .get("https://bugs.swift.org")
        .send()
        .and_then(|mut res| {
            println!("{}", res.status());

            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("request error: {}", err))
        .map(|body| {
            let mut body = Cursor::new(body);
            let _ = io::copy(&mut body, &mut io::stdout())
                .map_err(|err| {
                    println!("stdout error: {}", err);
                });
        })
}

fn main() {
    tokio::run(fetch());
}
