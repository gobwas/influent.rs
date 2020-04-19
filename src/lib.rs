extern crate futures;
extern crate http;
extern crate base64;
extern crate hyper;
extern crate reqwest;
extern crate url;

pub mod client;
pub mod hurl;
pub mod serializer;
pub mod measurement;

use client::Credentials;
use client::http::HttpClient;
use hurl::{hyper::HyperHurl, reqwest::{ReqwestHurl, ReqwestAsyncHurl}};
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
    let mut client = HttpClient::new(credentials,
                                     Box::new(LineSerializer::new()),
                                     Box::new(HyperHurl::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}

pub fn create_reqwest_client<'a>(credentials: Credentials<'a>,
                                 hosts: Vec<&'a str>) -> HttpClient<'a> {
    let mut client = HttpClient::new(credentials,
                                     Box::new(LineSerializer::new()),
                                     Box::new(ReqwestHurl::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}

pub fn create_async_reqwest_client<'a>(credentials: Credentials<'a>,
                                       hosts: Vec<&'a str>) -> HttpClient<'a> {
    let mut client = HttpClient::new(credentials,
                                     Box::new(LineSerializer::new()),
                                     Box::new(ReqwestAsyncHurl::new()));

    for host in hosts {
        client.add_host(host);
    }

    client
}
