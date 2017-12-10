extern crate serde_json;

use std;
use std::io;


#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Io(io::Error),
    Message(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(err: &str) -> Error {
        Error::Message(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
