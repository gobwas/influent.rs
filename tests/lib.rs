extern crate influent;
extern crate tokio;
extern crate futures;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::http::HttpClient;
use influent::measurement::{Measurement, Value};
use std::sync::Arc;

async fn before<'a>() -> HttpClient<'a> {
	let credentials = Credentials {
        username: "gobwas",
        password: "xxxx",
        database: "test"
    };

    let client = Arc::new(create_client(credentials, vec!["http://localhost:8086"]));
    client.query("drop database test".to_owned(), None)
        .await.expect("failed to drop");
    client.query("create database test".to_owned(), None)
        .await.expect("failed to create");

    if let Ok(client) = Arc::try_unwrap(client) {
        return client
    }
    unreachable!()
}

#[tokio::test]
async fn test_write_measurement() {
    let client = before().await;

    let mut measurement = Measurement::new("sut");

    measurement.add_field("string", Value::String("string"));
    measurement.add_field("integer", Value::Integer(10));
    measurement.add_field("float", Value::Float(10f64));
    measurement.add_field("boolean", Value::Boolean(false));
    measurement.add_field("with, comma", Value::String("comma, with"));

    measurement.add_tag("tag", "value");
    measurement.add_tag("tag, with comma", "three, four");

    measurement.set_timestamp(1_434_055_562_000_000_000);

    client.write_one(measurement, None)
        .await.expect("failed to write one");
    match client.query("select * from \"sut\"".to_owned(), None).await {
        Ok(res) => {
            // Response from InfluxDB 1.7.9
            let fixture = concat!(
                r#"{"results":[{"statement_id":0,"series":[{"name":"sut","columns""#,
                r#":["time","boolean","float","integer","string","tag","tag, with "#,
                r#"comma","with, comma"],"values":[["2015-06-11T20:46:02Z",false,1"#,
                r#"0,10,"string","value","three, four","comma, with"]]}]}]}"#, "\n"
            );
            assert_eq!(fixture, res);
        },
        Err(e) => panic!("{:?}", e),
    };
}
