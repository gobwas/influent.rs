use hyper::Client as HyperClient;
use hyper::Method as HyperMethod;
use hyper::Request as HyperRequest;
use http::header::AUTHORIZATION;
use url::Url;
use base64;
use futures::{self, Future, Stream};

use super::{Request, Response, Method, HurlResult};

use super::Hurl;

#[derive(Default)]
pub struct HyperHurl;

impl HyperHurl {
    pub fn new() -> HyperHurl {
        HyperHurl::default()
    }
}

impl Hurl for HyperHurl {
    fn request(&self, req: Request) -> HurlResult {
        let client = HyperClient::default();

        // map request method to the hyper's
        let method = match req.method {
            Method::POST => HyperMethod::POST,
            Method::GET  => HyperMethod::GET,
        };

        let mut url = match Url::parse(req.url) {
            Ok(u) => { u }
            Err(e) => {
                return Box::new(futures::future::err(format!("could not parse url: {:?}", e)));
            }
        };

        // if request has query
        if let Some(ref query) = req.query {
            // if any existing pairs
            let existing: Vec<(String, String)> = url.query_pairs().map(|(a,b)| (a.to_string(), b.to_string())).collect();

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
            url.query_pairs_mut().clear().extend_pairs(
                pairs.iter().map(|&(k, v)| { (&k[..], &v[..]) })
            );
        }

        // create query
        let mut query = HyperRequest::builder();
        query.method(method)
            .uri(url.as_str());

        // if request need to be authorized
        if let Some(auth) = req.auth {
            let auth = base64::encode(&format!("{}:{}", auth.username, auth.password));
            query.header(AUTHORIZATION, auth);
        }

        let request = if let Some(body) = req.body {
            query.body(body.into()).unwrap()
        } else {
            query.body("".into()).unwrap()
        };

        Box::new(client
            .request(request)
            .and_then(|resp| {
                let status = resp.status().as_u16();

                resp.into_body().concat2().and_then(move |body| {
                    Ok(String::from_utf8(body.to_vec()).unwrap())
                }).and_then(move |body|
                    Ok(Response {
                        status,
                        body
                    })
                )
            })
            .map_err(|_| format!(""))
        )
    }
}
