# ipify-cli
ipify-cli is a CLI command to know the public IP of the machine by using ipify API.
# Installation
## From Source Code
### Prerequisites
[git](https://git-scm.com/downloads)
,
[cargo](https://www.rust-lang.org/tools/install)
,
sudo

### Installation Procedure
```
git clone https://github.com/dvvvvvv/ipify-cli.git
cd ipify-cli
cargo build --release
sudo cp target/release/ipify-cli /usr/bin
```
# Usage
```
ipify-cli

USAGE:
    ipify-cli [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -v, --version <ip-version>    specify ip version [default: 4]  [possible values: 4, 6]
```
# EXAMPLE

```
# ipify-cli
98.207.254.136
# ipify-cli -v 4
98.207.254.136
# ipify-cli -v 6
2a00:1450:400f:80d::200e
```
