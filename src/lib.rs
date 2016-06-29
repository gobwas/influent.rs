#[macro_use] extern crate log;

pub mod client;
#[cfg(feature = "http")]
pub mod hurl;
pub mod serializer;
pub mod measurement;

use client::{Client, Credentials};
use client::udp::UdpClient;
#[cfg(feature = "http")]
use client::http::HttpClient;
#[cfg(feature = "http")]
use hurl::hyper::HyperHurl;
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
#[cfg(feature = "http")]
pub fn create_client<'a>(credentials: Credentials<'a>, hosts: Vec<&'a str>) -> HttpClient<'a> {
    let mut client = HttpClient::new(credentials, Box::new(LineSerializer::new()), Box::new(HyperHurl::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}

/// Simple factory of `UdpClient` with `LineSerializer`
/// Takes one parameter which is a host and port.
///
/// # Examples
/// ```
/// use influent::create_udp_client;
/// let client = create_udp_client(vec!["127.0.0.1:8089"]);
/// ```
pub fn create_udp_client<'a>(hosts: Vec<&'a str>) -> UdpClient<'a> {
    let mut client = UdpClient::new(Box::new(LineSerializer::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}
