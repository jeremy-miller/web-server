[![Build Status](https://travis-ci.org/jeremy-miller/webserver.svg?branch=master)](https://travis-ci.org/jeremy-miller/webserver)
[![codecov](https://codecov.io/gh/jeremy-miller/webserver/branch/master/graph/badge.svg)](https://codecov.io/gh/jeremy-miller/webserver)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jeremy-miller/webserver/blob/master/LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.23.0-blue.svg)]()

# Webserver
This webserver leverages a thread pool to serve simultaneous connections.
It is based on chapter 20 of the [Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/ch20-00-final-project-a-web-server.html).

## Table of Contents
- [Motivation](#motivation)
- [Prerequisites](#prerequisites)
- [Documentation](#documentation)
- [Build](#build)
- [Code Formatting](#code-formatting)
- [Static Code Analysis](#static-code-analysis)
- [Test](#test)
- [Run](#run)
- [License](#license)

## Motivation
I created this project while reading the [Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/).

## Prerequisites
This tool requires at least Rust version 1.23.0 to be installed.

## Documentation
To build and view the `webserver` documentation in your browser, execute the following command:
```cargo doc --open```

## Build
To build `webserver`, execute the following command:
```cargo build --all```

## Code Formatting
To run `rustfmt` on `webserver`, execute the following steps:

1. Install `rustfmt` (requires rust `nightly`): ```rustup component add rustfmt-preview --toolchain=nightly```
2. Run `rustfmt` on the `minigrep` codebase: ```cargo +nightly fmt --all```

## Static Code Analysis
To run `clippy` on `webserver`, execute the following steps:

1. Install `clippy` (requires rust `nightly`): ```cargo +nightly install clippy```
2. Run `clippy` on the `minigrep` codebase: ```cargo +nightly clippy --all```

## Test
To run the `webserver` tests, execute the following command:
```cargo test --all```

## Run
To run the `webserver`, execute the following steps:

1. Run the `webserver`: ```cargo run```
2. Navigate to any of the following URLs in your web browser.
    - `localhost:8080/` - returns a "success" page
    - `localhost:8080/sleep` - worker sleeps for five seconds before responding
    - any other URI - returns a 404 page

## License
[MIT](https://github.com/jeremy-miller/webserver/blob/master/LICENSE)
