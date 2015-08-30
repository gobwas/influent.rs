# influent.[rs](https://www.rust-lang.org/)

[![Build Status][travis-image]][travis-url] [![crates.io][crates-image]][crates-url]

> [InfluxDB](https://influxdb.com/) rust package

## Overview

This is a InfluxDB driver for Rust apps.

## Install

```toml
// Cargo.toml
[dependencies]
influent = "0.1"
```

## Usage

```rust
use influent::create_client;
use influent::client::Credentials;
use influent::measurement::{Measurement, Value};

let credentials = Credentials {
    username: "gobwas",
    password: "xxx",
    database: "mydb"
};

let hosts = vec!["http://localhost:8086"];

let client = create_client(credentials, hosts);

let mut measurement = Measurement::new("key");

measurement.add_field("field", Value::String("hello"));

client.write_one(measurement, None);
```

## License

MIT © [Sergey Kamardin](https://github.com)

[travis-image]: https://travis-ci.org/gobwas/influent.rs.svg?branch=master
[travis-url]: https://travis-ci.org/gobwas/influent.rs
[crates-image]: http://meritbadge.herokuapp.com/influent
[crates-url]: https://crates.io/crates/influent