use std::collections::HashMap;

pub mod hyper;

pub trait Hurl {
	fn request(&self, Request) -> HurlResult;
}

pub struct Request<'a> {
	pub url: &'a str,
	pub method: Method,
	pub auth: Option<Auth<'a>>,
	pub query: Option<HashMap<&'a str, String>>,
	pub body: Option<String>
}

pub struct Response {
	status: u16,
	body: String
}

pub type HurlResult = Result<Response, String>;

pub enum Method {
	POST,
	GET
}

pub struct Auth<'a> {
	pub username: &'a str,
	pub password: &'a str
}