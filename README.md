# simple_logger [![](https://img.shields.io/github/tag/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/tags) [![](https://img.shields.io/travis/borntyping/rust-simple_logger.svg)](https://travis-ci.org/borntyping/rust-simple_logger) [![](https://img.shields.io/github/issues/borntyping/rust-simple_logger.svg)](https://github.com/borntyping/rust-simple_logger/issues)

A logger that prints all messages and uses an output format similar to [Supervisord](http://supervisord.org/).

Future updates may include coulored output based on the log level of the message and selecting a `max_log_level` based on the value of an input string (e.g. from a flag parsed by [docopt](https://github.com/docopt/docopt.rs)).

Usage
-----

```rust
#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init();

    warn!("This is an example message.");
}
```

This outputs:

```
2015-02-24 01:05:20 WARN [logging_example] This is an example message.
```

You can run the above example with:

```bash
cargo run --example logging_example
```

Licence
-------

`simple_logger` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
