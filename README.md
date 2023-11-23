# simple_logger [![](https://img.shields.io/github/tag/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/tags) [![](https://img.shields.io/travis/borntyping/rust-simple_logger.svg)](https://travis-ci.org/borntyping/rust-simple_logger) [![](https://img.shields.io/github/issues/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/issues)

A logger that prints all messages with a readable output format.

The output format is based on the format used by [Supervisord](https://github.com/Supervisor/supervisor), with timestamps default [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) format. The format used for timestamps can be customised.

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

```txt
2022-01-19T17:27:07.013874956Z WARN [logging_example] This is an example message.
```

You can run the above example with:

```sh
cargo run --example init
```

The `colors` and `timestamps` features are enabled by default. You can remove these
features and their respective dependencies by disabling all features in your
`Cargo.toml`.

```toml
[dependencies.simple_logger]
default-features = false
```

To include the `timestamps` feature, but not the `colors` feature:

```toml
[dependencies.simple_logger]
default-features = false
features = ["timestamps"]
```

To include the `colors` feature, but not the `timestamps` feature:

```toml
[dependencies.simple_logger]
default-features = false
features = ["colors"]
```

To include thread metadata use the `threads` and `nightly` features:

```toml
[dependencies.simple_logger]
features = ["threads", "nightly"]
```

To direct logging output to `stderr` use the `stderr` feature:

```toml
[dependencies.simple_logger]
features = ["stderr"]
```

Multiple features can be combined.

```toml
[dependencies.simple_logger]
features = ["colors", "threads", "timestamps", "nightly", "stderr"]
```

Wrapping with another logger
----------------------------

Users that might want to wrap this logger to be able to catch log events for various 
reasons can setup the logger as follows:

On windows machines:
```rust
let logger = SimpleLogger::new();
set_up_color_terminal();
let max_level = logger.max_level(); 
```

Otherwise:
```rust
let logger = SimpleLogger::new();
let max_level = logger.max_level();
```

The user can then themselves call `log::set_max_level` and `log::set_boxed_logger` or equivalent as they wish.

Licence
-------

`simple_logger` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
