#[macro_use] extern crate log;
extern crate hyper;

pub mod client;
pub mod serializer;
pub mod measurement;

pub enum InfluentError {
    Unknown
}
