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
- [Docker](https://docs.docker.com/engine/installation/)

## Documentation

## Build
```cargo build --all```
``````docker build -t jeremymiller/minigrep .``````

## Code Formatting

## Static Code Analysis

## Test
```cargo test --all```
```docker run -it --rm jeremymiller/minigrep cargo test --all```

## Run
```cargo run```

## License
[MIT](https://github.com/jeremy-miller/webserver/blob/master/LICENSE)
