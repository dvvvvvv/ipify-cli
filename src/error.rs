pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
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
