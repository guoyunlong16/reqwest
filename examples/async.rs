extern crate futures;
extern crate futures_cpupool;
extern crate reqwest;
extern crate tokio;
#[macro_use]
extern crate lazy_static;

use std::mem;
use std::io::{self, Cursor};
use futures::{Future, Stream};
use reqwest::async::{ClientBuilder, Decoder};
use futures_cpupool::{CpuPool, Builder};

lazy_static! {
    static ref CPU_POOL: CpuPool = Builder::new()
        .pool_size(4)
        .name_prefix("t:dns-")
        .create();
}

fn fetch() -> impl Future<Item=(), Error=()> {
    let client_builder = ClientBuilder::new_with_cpupool(CPU_POOL.clone());
    let client = client_builder
        .danger_accept_invalid_certs(true)
        .http_version(reqwest::HttpVersion::Http11)
        .use_default_tls()
        //.use_rustls_tls()
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
            // std::thread::sleep(std::time::Duration::from_secs(1000000));
            let _ = io::copy(&mut body, &mut io::stdout())
                .map_err(|err| {
                    println!("stdout error: {}", err);
                });
        })
}

fn main() {
    tokio::run(fetch());
}
