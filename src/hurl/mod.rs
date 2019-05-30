use std::collections::HashMap;
use futures::Future;

pub mod hyper;

pub trait Hurl {
    fn request(&self, Request) -> HurlResult;
}

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub auth: Option<Auth>,
    pub query: Option<HashMap<String, String>>,
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
pub struct Auth {
    pub username: String,
    pub password: String
}
