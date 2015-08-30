#[macro_use] extern crate log;

pub mod client;
pub mod serializer;
pub mod measurement;

use client::{Client, Credentials};
use client::http::HttpClient;
use client::http::hurl::hyper::HyperHurl;
use serializer::Serializer;
use serializer::line::LineSerializer;

/// Simple factory of `HttpClient` with `LineSerializer`
///
/// Takes two parameters, where first is `Credentials` struct, and second - `Vec<&str>`, where each item
/// is a InfluxDB host url.
///
/// # Examples
///
/// ```
/// use influent::create_client;
/// use influent::client::Credentials;
///
/// let credentials = Credentials {
///     username: "gobwas",
///     password: "xxx",
///     database: "mydb"
/// };
///
/// let client = create_client(credentials, vec!["http://localhost:8086"]);
/// ```
pub fn create_client<'a>(credentials: Credentials<'a>, hosts: Vec<&'a str>) -> HttpClient<'a> {
    let mut client = HttpClient::new(credentials, Box::new(LineSerializer::new()), Box::new(HyperHurl::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}