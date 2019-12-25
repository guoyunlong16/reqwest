#![deny(warnings)]
use reqwest::ClientBuilder;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client_builder = ClientBuilder::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate"));
    headers.insert(header::CONNECTION, header::HeaderValue::from_static("keep-alive"));

    // use charles to capture http2 headers
    let proxy_url = "http://127.0.0.1:8888";
    let reqwest_proxy = reqwest::Proxy::all(proxy_url).unwrap();
    let client = client_builder
        .danger_accept_invalid_certs(true)
        .proxy(reqwest_proxy)
        .default_headers(headers)
        .http2_prior_knowledge()
        .use_rustls_tls()
        .build()
        .unwrap();

    let res = client
            .get("https://hyper.rs")
            .send().await?;

    println!("Status: {}", res.status());

    Ok(())
}
