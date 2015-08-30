extern crate influent;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::client::http::hurl::hyper::HyperHurl;

#[test]
fn test_create_client() {
	let credentials = Credentials {
		username: "gobwas",
		password: "xxx",
		database: "mydb"
	};

	create_client(credentials, vec!["http://localhost:8086"]);
}