use hyper::body::{self, Body};
use hyper::client::{Client, HttpConnector};
use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use std::convert::From;

type Result<T> = std::result::Result<T, Error>;

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
    match ip(client, IpVersion::V4).await {
        Ok(ip) => println!("{}", ip),
        Err(err) => eprintln!("{}", err),
    }
}

enum IpVersion {
    V4,
    V6,
}

async fn ip<T>(client: Client<T, Body>, ip_version: IpVersion) -> Result<String>
where
    T: hyper::client::connect::Connect,
    T: std::clone::Clone,
    T: std::marker::Send,
    T: std::marker::Sync,
    T: 'static,
{
    let body = body::to_bytes(client.get(uri(ip_version)).await?.into_body()).await?;
    Ok(String::from_utf8_lossy(&body).to_string())
}

fn uri(ip_version: IpVersion) -> Uri {
    match ip_version {
        IpVersion::V4 => "https://api.ipify.org".parse().unwrap(),
        IpVersion::V6 => "https://api6.ipify.org".parse().unwrap(),
    }
}

fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
    Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new())
}

#[cfg(test)]
mod test {
    use super::*;

    fn ip_v4_api_uri() {
        let uri = uri(IpVersion::V4);
        assert_eq!("https://api.ipify.org".parse::<Uri>().unwrap(), uri)
    }

    fn ip_v6_api_uri() {
        let uri = uri(IpVersion::V6);
        assert_eq!("https://api6.ipify.org".parse::<Uri>().unwrap(), uri)
    }
}
