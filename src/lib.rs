#[macro_use] extern crate log;

pub mod client;
pub mod serializer;
pub mod measurement;

pub enum InfluentError {
    Unknown
}
