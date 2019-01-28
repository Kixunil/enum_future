//! This crate contains a simple macro for creating `Future`-implementing enums containing futures.
//!
//! It can be used as an alternative to `futures::Either` if more than two variants are desired or
//! if one wants to give them more meaningful names. The enum is generic, in order to allow use
//! with complicated or unname-able types (combinators, closures, impl Future).
//!
//! This macro doesn't allow for creating `pub` enums, documenting them or deriving any other
//! trait. This should be fine in most cases, as it's intended to be used mainly in `match`
//! expressions returning different futures.
//!
//! Derive macro would certainly provide more flexibility, but it'd also be more complicated and
//! this simple macro solves my problem. If anyone is interested in contributing proc macro
//! version, I'd be very happy to merge a PR!
//!
//! This crate is `no_std` and pre-2018-compatible, since it doesn't need anything fancy from
//! neither `std` nor 2018 edition.
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate enum_future;
//!
//! use enum_future::futures::future::{Future, ok, err};
//!
//! fn make_future(input: u64) -> impl Future<Item=bool, Error=&'static str> {
//!     enum_future!(Ret, Zero, One, Other);
//!
//!     match input {
//!         0 => Ret::Zero(ok(false)),
//!         1 => Ret::One(ok(true)),
//!         _ => Ret::Other(err("Invalid input")),
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(make_future(42).wait(), Err("Invalid input"));
//! }
//! ```

#![no_std]

#[cfg(test)]
extern crate std;
pub extern crate futures;

/// Creates a generic enum of type `$name` with variants `$first, $($other,)*`, each containing a
/// single value of possibly different type. The enum implements `Future` by delegating to the
/// value of appropriate variant.
#[macro_export]
macro_rules! enum_future {
    ($name:ident, $first:ident, $($other:ident),*) => {
        enum $name<$first, $($other,)*> {
            $first($first),
            $(
                $other($other),
            )*
        }

        impl<$first, $($other,)*> $crate::futures::Future for $name<$first, $($other,)*> where 
            $first: $crate::futures::Future,
            $($other: $crate::futures::Future<Item=$first::Item, Error=$first::Error>,)*
        {
            type Item=$first::Item;
            type Error=$first::Error;

            fn poll(&mut self) -> $crate::futures::Poll<Self::Item, Self::Error> {
                match self {
                    $name::$first(future) => future.poll(),
                    $(
                        $name::$other(future) => future.poll(),
                    )*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(unused)]
    fn it_works() {
        use ::futures::Future;

        enum_future!(TestEnum, Foo, Bar, Baz);

        let test: TestEnum<_, futures::future::FutureResult<_, ()>, futures::future::FutureResult<_, _>> = TestEnum::Foo(::futures::future::ok("Hello world!"));
        assert_eq!(test.wait().unwrap(), "Hello world!");
    }
}
