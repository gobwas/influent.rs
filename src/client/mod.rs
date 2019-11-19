use async_trait::async_trait;
use crate::measurement::Measurement;
use std::io;

pub mod http;

#[async_trait]
pub trait Client {
    async fn write_many(&self, line: &[Measurement<'_>], precision: Option<Precision>) -> Result<(), ClientError>;
    async fn write_one(&self, line: Measurement<'_>, precision: Option<Precision>) -> Result<(), ClientError>;
    async fn query(&self, query: String, precision: Option<Precision>) -> Result<String, ClientError>;
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
