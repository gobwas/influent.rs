# influent.[rs](https://www.rust-lang.org/)

[![Build Status][travis-image]][travis-url] [![crates.io][crates-image]][crates-url]

> [InfluxDB](https://www.influxdata.com/) Rust package

## Overview

This is an InfluxDB driver for Rust.

## Status

Library **is not under active development** right now.

PRs are welcome and merged from time to time.
If you want to become a collaborator of this library please let me know.

## Install

> Cargo.toml

```toml
[dependencies]
influent = "0.5"
```

## Usage

```rust
extern crate influent;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::measurement::{Measurement, Value};

// prepare client
let credentials = Credentials {
    username: "gobwas",
    password: "xxx",
    database: "mydb"
};
let hosts = vec!["http://localhost:8086"];
let client = create_client(credentials, hosts);

// prepare measurement
let mut measurement = Measurement::new("key");
measurement.add_field("some_field", Value::String("hello"));
measurement.add_tag("some_region", "Moscow");

client.write_one(measurement, None);
```

## Documentation

API documentation placed [here](http://gobwas.github.io/influent.rs/influent/index.html).

## Compatibility

This is a table of InfluxDB [write spec](https://influxdb.com/docs/v0.9/write_protocols/write_syntax.html) compatibility respectively to Influent version:

InfluxDB | Influent
---------|---------
`0.9.2`  | `^0.1.0`
`0.9.3`  | `^0.2.0`

## License

MIT © [Sergey Kamardin](https://github.com/gobwas)

[travis-image]: https://travis-ci.org/gobwas/influent.rs.svg?branch=master
[travis-url]: https://travis-ci.org/gobwas/influent.rs
[crates-image]: http://meritbadge.herokuapp.com/influent
[crates-url]: https://crates.io/crates/influent
