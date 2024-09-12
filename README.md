[![Build](https://github.com/whs31/renga-api-rs/actions/workflows/build.yml/badge.svg)](https://github.com/whs31/renga-api-rs/actions/workflows/build.yml)[![Latest version](https://img.shields.io/crates/v/renga-api-rs.svg)](https://crates.io/crates/renga-api-rs)[![Docs](https://github.com/whs31/renga-api-rs/actions/workflows/docs.yml/badge.svg)](https://github.com/whs31/renga-api-rs/actions/workflows/docs.yml)![License](https://img.shields.io/crates/l/renga-api-rs.svg)

This crate provides (*incomplete*) Rust bindings for the [Renga](https://www.rengabim.com) COM API.

> Only Windows is supported at the moment.

### Logging
This crate uses `log` crate interface to log messages.
You must provide your own logger implementation in order to see logs.

See [log crate](https://crates.io/crates/log) for more details.

### Compatibility
Rust version at least **1.65** is required.
This crate is compatible with Renga **8.0.0** and higher.

### Links
See [Official documentation](https://help.rengabim.com/) for more details about Renga or 
[Crate documentation](https://whs31.github.io/renga-api-rs/) for more details about this crate.