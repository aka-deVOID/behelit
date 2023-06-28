use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("utf8 error")]
    Utf8(#[from] std::str::Utf8Error),
}
