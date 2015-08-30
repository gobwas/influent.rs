extern crate hyper;

use self::hyper::Client as HyperClient;
use self::hyper::method::Method as HyperMethod;
use self::hyper::client::Body;
use self::hyper::Url;
use self::hyper::header::Connection;
use self::hyper::header::{Headers, Authorization, Basic};

use super::{Request, Response, Method, Auth, HurlResult};
use std::io::Read;

use super::Hurl;

pub struct HyperHurl;

impl HyperHurl {
    pub fn new() -> HyperHurl {
        HyperHurl
    }
}

impl Hurl for HyperHurl {
    fn request(&self, req: Request) -> HurlResult {
        let mut client = HyperClient::new();

        // map request method to the hyper's
        let method = match req.method {
            Method::POST => HyperMethod::Post,
            Method::GET  => HyperMethod::Get
        };

        let mut headers = Headers::new();

        let mut url = match Url::parse(req.url) {
            Ok(u) => { u }
            Err(e) => {
                return Err(format!("could not parse url: {:?}", e));
            }
        };

        // if request need to be authorized
        match req.auth {
            Some(auth) => {
                headers.set(
                   Authorization(
                       Basic {
                           username: auth.username.to_string(),
                           password: Some(auth.password.to_string())
                       }
                   )
                );
            }
            _ => {}
        };

        // if request has query
        match req.query {
            Some(ref query) => {
                // if any existing pairs
                let existing: Vec<(String, String)> = match url.query_pairs() {
                    Some(ref existing) => {
                        existing.clone()
                    }
                    _ => {
                        Vec::new()
                    }
                };

                // final pairs
                let mut pairs: Vec<(&str, &str)> = Vec::new();

                // add first existing
                for pair in &existing {
                    pairs.push((&pair.0, &pair.1));
                }

                // add given query to the pairs
                for (key, val) in query.iter() {
                    pairs.push((key, val));
                }

                // set new pairs
                url.set_query_from_pairs(pairs.into_iter());
            }
            _ => {}
        };

        // create query
        let mut query = client.request(method, url).headers(headers);
        
        // if request has body
        query = match req.body {
            Some(ref body) => {
                query.body(body)
            }
            None => { query }
        };

        // go!
        match query.send() {
            Ok(ref mut resp) => {
                let mut body = String::new();
                resp.read_to_string(&mut body).unwrap();

                Ok(Response {
                    status: resp.status.to_u16(),
                    body: body
                })
            }
            Err(err) => {
                Err(format!("something went wrong: {:?}", err))
            }
        }
    }
}