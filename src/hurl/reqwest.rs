use reqwest::Client;
use reqwest::async::Client as ClientAsync;
use reqwest::Method as ReqwestMethod;
use url::Url;
use futures::future::{err as ferr, ok as fok};
use futures::{Future, Stream};

use super::{Request, Response, Method, HurlResult};

use super::Hurl;

macro_rules! craft_request {
    ($client:ident, $req:ident) => {{
        // map request method to the reqwest's
        let method = match $req.method {
            Method::POST => ReqwestMethod::POST,
            Method::GET  => ReqwestMethod::GET,
        };

        let mut url = match Url::parse($req.url) {
            Ok(u) => { u }
            Err(e) => {
                return Box::new(ferr(format!("could not parse url: {:?}", e)));
            }
        };

        // if request has query
        if let Some(ref query) = $req.query {
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
        let query = $client.request(method, url.as_str());

        // if request need to be authorized
        let query = if let Some(auth) = $req.auth {
            query.basic_auth(auth.username, Some(auth.password))
        } else {
            query
        };

        let query = if let Some(body) = $req.body {
            query.body(body)
        } else {
            query.body("")
        };

        query.build().unwrap()
    }}
}

#[derive(Default)]
pub struct ReqwestHurl;

impl ReqwestHurl {
    pub fn new() -> ReqwestHurl {
        ReqwestHurl::default()
    }
}

impl Hurl for ReqwestHurl {
    fn request(&self, req: Request) -> HurlResult {
        let client = Client::new();

        let request = craft_request!(client, req);

        Box::new(match client.execute(request) {
            Ok(mut resp) => {
                let status = resp.status().as_u16();
                let body = match resp.text() {
                    Ok(s) => s,
                    Err(_) => "".to_string()
                };
                fok(Response { status, body })
            },
            Err(e) => ferr(format!("{:?}", e))
        })
    }
}

#[derive(Default)]
pub struct ReqwestAsyncHurl;

impl ReqwestAsyncHurl {
    pub fn new() -> ReqwestAsyncHurl {
        ReqwestAsyncHurl::default()
    }
}

impl Hurl for ReqwestAsyncHurl {
    fn request(&self, req: Request) -> HurlResult {
        let client = ClientAsync::new();
        let request = craft_request!(client, req);

        Box::new(client
            .execute(request)
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
            .map_err(|e| format!("{:?}", e))
        )
    }
}
