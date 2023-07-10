# IP2GEO

Easy IP geo location lookup from the command line, using ipgeolocation.io (you will need to setup a free account and get an API key)

## Building

```bash
cargo build
```

Then move the binary file to `/usr/local/bin` and your ip2geo command is ready to use once you reset your terminal session.

## Tests

TODO: will need to cover this with tests!

You can run tests using `cargo test`

## Usage

Before you start, you'll need to create a dot file in your home directory `.ip2geo`, adding your API key in there.

This tool provides a single functionality; you pass an IP address, it responds with the city + country that IP belongs to.

## Why

While it will certainly come in handy, my motivation to write this tool was mainly to learn Rust and get comfortable with concepts like borrowing, ownership, structs, match expressions, utilising the Result type, etc.

## License

This code is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
