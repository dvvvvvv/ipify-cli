use clap::{App, Arg};
use hyper::body::{self, Body};
use hyper::client::{Client, HttpConnector};
use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use std::convert::From;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    HyperError(hyper::error::Error),
    UnsupportedIpVersion(String),
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
            Error::UnsupportedIpVersion(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HyperError(hyper_error) => hyper_error.fmt(formatter),
            Error::UnsupportedIpVersion(version) => {
                write!(formatter, "UnsupportedIpVersion({})", version)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = app().get_matches();
    let ip_version: IpVersion = match matches.value_of("ip-version").unwrap_or("6").parse() {
        Ok(version) => version,
        Err(err) => exit_with_error(err),
    };
    let client = IpifyClient::default();
    match client.ip(ip_version).await {
        Ok(ip) => println!("{}", ip),
        Err(err) => exit_with_error(err),
    }
}

fn exit_with_error(error: Error) -> ! {
    eprintln!("closed with error: {}", error);
    std::process::exit(-1)
}

fn app() -> App<'static, 'static> {
    App::new("ipify-cli").arg(
        Arg::with_name("ip version")
            .short("v")
            .long("version")
            .value_name("ip-version")
            .help("specify ip version")
            .takes_value(true)
            .default_value("4")
            .possible_values(&["4", "6"]),
    )
}

enum IpVersion {
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

struct IpifyClient<T> {
    http_client: Client<T, Body>,
}

impl Default for IpifyClient<HttpsConnector<HttpConnector>> {
    fn default() -> IpifyClient<HttpsConnector<HttpConnector>> {
        Self::with_connector(hyper_tls::HttpsConnector::new())
    }
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

fn uri(ip_version: IpVersion) -> Uri {
    match ip_version {
        IpVersion::V4 => "https://api.ipify.org".parse().unwrap(),
        IpVersion::V6 => "https://api6.ipify.org".parse().unwrap(),
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
