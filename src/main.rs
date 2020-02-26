use hyper::body::{self, Body};
use hyper::client::{Client, HttpConnector};
use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    HyperError(hyper::error::Error),
}

impl From<hyper::error::Error> for Error {
    fn from(error: hyper::error::Error) -> Error {
        Error::HyperError(error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::HyperError(hyper_error) => Some(hyper_error),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HyperError(hyper_error) => hyper_error.fmt(formatter),
        }
    }
}

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

enum IpVersion {
    V4,
    V6,
}

//async fn ip(client: Client<_,Body>, ip_version: IpVersion) -> String {
//}

fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
    Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new())
}
