use hyper::client::{Client, HttpConnector};
use hyper::http::Uri;
use hyper::body::{self, Body};
use hyper_tls::HttpsConnector;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let client = client();
    let call = client.get(Uri::from_str("https://api.ipify.org").unwrap());
    let response = call.await.unwrap();
    let body = response.into_body();
    let bytes = body::to_bytes(body).await.unwrap();
    let ip = String::from_utf8_lossy(&bytes);
    println!("{}", ip);
}

fn client() -> Client<HttpsConnector<HttpConnector>, Body>{
    Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new())
}

