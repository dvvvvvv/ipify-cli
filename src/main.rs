use clap::{App, Arg};

use client::{IpVersion, IpifyClient};
pub use error::{Error, Result};

mod client;
mod error;

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
