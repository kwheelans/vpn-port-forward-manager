use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authorization request unsuccessful")]
    Authorization,

    #[error("Unsuccessful response: {0}")]
    AppResponse(String),

    #[error("Parsing failed: {0}")]
    ParsingFailure(String),

    #[error("Port forward path not accessible: {0}")]
    PortPath(String),

    #[error("Port update unsuccessful: {0}")]
    PortUpdate(String),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Parse Integer Error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Reqwest Error: {0:?}")]
    Reqwest(#[from] reqwest::Error),
}
