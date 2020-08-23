[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)
[![Build Status](https://travis-ci.com/realaravinth/amnesia.svg?branch=master)](https://travis-ci.com/realaravinth/amnesia)
[![License: GPLv2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
# Amnesia

a very forgetful static-file echo server written in Rust

## Build dependencies
- [Cargo](https://github.com/rust-lang/cargo#compiling-from-source)

## Installation


`$ git clone https://github.com/realaravinth/amnesia`

`$ cd amnesia && cargo build --release`

## Initializing

` $ target/release/amnesia`

The server by default listens on `PORT 3000`, this can be changed by
setting `$PORT` environment variable and passing it like so:

`$ target/release/amnesia $PORT`


## Usage

The server accepts `multipart/form-data` at `/archive` and will serve
the received data at `/fetch`.

- ##### Uploading
	+ With curl

		`$ curl --location --request POST localhost:3000/archive/' \
--form 'name=@/path/to/file'`
