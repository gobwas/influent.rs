use ::measurement::Measurement;
use std::io;
use futures::Future;

pub mod http;

pub trait Client {
    fn write_many(&self, &[Measurement], Option<Precision>) -> ClientWriteResult;
    fn write_one(&self, Measurement, Option<Precision>) -> ClientWriteResult;
    fn query(&self, String, Option<Precision>) -> ClientReadResult;
}

pub struct Credentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub database: &'a str
}

pub enum Precision {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours
}

impl ToString for Precision {
    fn to_string(&self) -> String {
        let s = match *self {
            Precision::Nanoseconds  => "n",
            Precision::Microseconds => "u",
            Precision::Milliseconds => "ms",
            Precision::Seconds      => "s",
            Precision::Minutes      => "m",
            Precision::Hours        => "h"
        };

        s.to_string()
    }
}

pub type ClientWriteResult = Box<Future<Item=(), Error=ClientError> + Send>;

// TODO: here parsing json?
pub type ClientReadResult = Box<Future<Item=String, Error=ClientError> + Send>;

#[derive(Debug)]
pub enum ClientError {
    CouldNotComplete(String),
    Communication(String),
    Syntax(String),
    Unexpected(String),
    Unknown
}

impl From<io::Error> for ClientError {
    fn from(e: io::Error) -> Self {
        ClientError::Communication(format!("{}", e))
    }
}
