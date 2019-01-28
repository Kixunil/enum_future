enum future
===========

A simple macro for creating `Future`-implementing enums containing futures.

About
-----
The `enum_future` macro can be used as an alternative to `futures::Either` if more than two variants
are desired or if one wants to give them more meaningful names. The enum is generic, in order to allow use with complicated or unname-able types (combinators, closures, impl Future).

This macro doesn't allow for creating `pub` enums, documenting them or deriving any other
trait. This should be fine in most cases, as it's intended to be used mainly in `match`
expressions returning different futures.

Derive macro would certainly provide more flexibility, but it'd also be more complicated and
this simple macro solves my problem. If anyone is interested in contributing proc macro
version, I'd be very happy to merge a PR!

This crate is `no_std` and pre-2018-compatible, since it doesn't need anything fancy from neither `std`
nor 2018 edition.

Example
-------

```rust
#[macro_use]
extern crate enum_future;

use enum_future::futures::future::{Future, ok, err};

fn make_future(input: u64) -> impl Future<Item=bool, Error=&'static str> {
    enum_future!(Ret, Zero, One, Other);

    match input {
        0 => Ret::Zero(ok(false)),
        1 => Ret::One(ok(true)),
        _ => Ret::Other(err("Invalid input")),
    }
}

fn main() {
    assert_eq!(make_future(42).wait(), Err("Invalid input"));
}
```

License
-------

MITNFA
