use std::collections::HashMap;
use futures::Future;

pub mod hyper;

pub trait Hurl {
    fn request(&self, Request) -> HurlResult;
}

#[derive(Debug)]
pub struct Request<'a> {
    pub url: &'a str,
    pub method: Method,
    pub auth: Option<Auth<'a>>,
    pub query: Option<HashMap<&'a str, String>>,
    pub body: Option<String>
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.body.clone()
    }
}

pub type HurlResult = Box<Future<Item=Response, Error=String> + Send>;

#[derive(Debug)]
pub enum Method {
    POST,
    GET
}

#[derive(Debug)]
pub struct Auth<'a> {
    pub username: &'a str,
    pub password: &'a str
}
