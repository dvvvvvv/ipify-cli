use super::{Error, Result};
use hyper::body::{self, Body};
use hyper::client::{Client, HttpConnector};
use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use std::str::FromStr;

pub struct IpifyClient<T> {
    http_client: Client<T, Body>,
}

impl<T> IpifyClient<T>
where
    T: hyper::client::connect::Connect,
    T: std::clone::Clone,
    T: std::marker::Send,
    T: std::marker::Sync,
    T: 'static,
{
    pub fn with_connector(connector: T) -> Self {
        Self::with_http_client(Client::builder().build::<_, Body>(connector))
    }

    pub fn with_http_client(http_client: Client<T, Body>) -> Self {
        IpifyClient { http_client }
    }

    pub async fn ip(&self, ip_version: IpVersion) -> Result<String> {
        let body = body::to_bytes(self.http_client.get(uri(ip_version)).await?.into_body()).await?;
        Ok(String::from_utf8_lossy(&body).to_string())
    }
}

impl Default for IpifyClient<HttpsConnector<HttpConnector>> {
    fn default() -> IpifyClient<HttpsConnector<HttpConnector>> {
        Self::with_connector(hyper_tls::HttpsConnector::new())
    }
}

fn uri(ip_version: IpVersion) -> Uri {
    match ip_version {
        IpVersion::V4 => "https://api.ipify.org".parse().unwrap(),
        IpVersion::V6 => "https://api6.ipify.org".parse().unwrap(),
    }
}

pub enum IpVersion {
    V4,
    V6,
}

impl FromStr for IpVersion {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "4" => Ok(IpVersion::V4),
            "6" => Ok(IpVersion::V6),
            _ => Err(Error::UnsupportedIpVersion(input.to_owned())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ip_v4_api_uri() {
        let uri = uri(IpVersion::V4);
        assert_eq!("https://api.ipify.org".parse::<Uri>().unwrap(), uri)
    }

    #[test]
    fn ip_v6_api_uri() {
        let uri = uri(IpVersion::V6);
        assert_eq!("https://api6.ipify.org".parse::<Uri>().unwrap(), uri)
    }
}
