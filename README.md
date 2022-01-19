# simple_logger [![](https://img.shields.io/github/tag/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/tags) [![](https://img.shields.io/travis/borntyping/rust-simple_logger.svg)](https://travis-ci.org/borntyping/rust-simple_logger) [![](https://img.shields.io/github/issues/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/issues)

A logger that prints all messages with a readable output format.

The output format is based on the format used by [Supervisord](http://supervisord.org/), with timestamps in [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) format.

* [Source on GitHub](https://github.com/borntyping/rust-simple_logger)
* [Packages on Crates.io](https://crates.io/crates/simple_logger)
* [Documentation on Docs.rs](https://docs.rs/simple_logger)

Breaking changes
----------------

- **Version 2.0.0 changes the default from displaying timestamps in the local timezone to displaying timestamps in UTC.** See issue [#52](https://github.com/borntyping/rust-simple_logger/issues/52) for more information.

Usage
-----

```rust
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::warn!("This is an example message.");
}
```

This outputs:

```
2022-01-19T17:27:07.013874956Z WARN [logging_example] This is an example message.
```

You can run the above example with:

```bash
cargo run --example init
```

Coloured output and timestamps will be enabled by default. You can remove these
features and their respective dependencies by disabling all features in your
`Cargo.toml`.

```
[dependencies.simple_logger]
default-features = false
```

To include the `timestamps` feature, but not the `colors` feature:

```
[dependencies.simple_logger]
default-features = false
features = ["timestamps"]
```

To include the `colors` feature, but not the `timestamps` feature:

```
[dependencies.simple_logger]
default-features = false
features = ["colors"]
```

To direct logging output to `stderr` use the `stderr` feature:

```
[dependencies.simple_logger]
features = ["stderr"]
```

This can be combined with any other feature.

Licence
-------

`simple_logger` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
