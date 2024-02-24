#![no_std]

//! Annotates a function that "throws" a Result.
//!
//! Inside functions tagged with either `throws` or `try_fn`, you can use `?` and the `throw!`
//! macro to return errors, but you don't need to wrap the successful return values in `Ok`.
//!
//! Using this syntax, you can write fallible functions almost as if they were infallible. Every
//! time a function call would return a `Result`, you "re-raise" the error using `?`, and if you
//! wish to raise your own error, you can return it with the `throw!` macro.
//!
//! The difference between `throws` and `try_fn` is in the function signature, with `throws` you
//! write the signature as if it were infallible too, it will be transformed into a `Result` for
//! you. With `try_fn` you write the signature as normal and only the body of the function will be
//! transformed.
//!
//! ## Example
//!
//! ```
//! use std::io::{self, Read};
//!
//! use culpa::{throw, throws, try_fn};
//!
//! #[throws(io::Error)]
//! fn check() {
//!     let mut file = std::fs::File::open("The_House_of_the_Spirits.txt")?;
//!     let mut text = String::new();
//!     file.read_to_string(&mut text)?;
//!
//!     if !text.starts_with("Barrabas came to us by sea, the child Clara wrote") {
//!         throw!(io::Error::from_raw_os_error(22));
//!     }
//!
//!     println!("Okay!");
//! }
//!
//! #[try_fn]
//! fn check_as_try_fn() -> std::io::Result<()> {
//!     let mut file = std::fs::File::open("The_House_of_the_Spirits.txt")?;
//!     let mut text = String::new();
//!     file.read_to_string(&mut text)?;
//!
//!     if !text.starts_with("Barrabas came to us by sea, the child Clara wrote") {
//!         throw!(io::Error::from_raw_os_error(22));
//!     }
//!
//!     println!("Okay!");
//! }
//! ```
//!
//! # `throws` Default Error Type
//!
//! The `throws` macro supports a "default error type" - if you do not pass a type to the macro, it
//! will use the type named `Error` in the current scope. So if you have defined an error type in
//! the module, that will be the error thrown by this function.
//!
//! You can access this feature by omitting the arguments entirely or by passing `_` as the type.
//!
//! ## Example
//!
//! ```
//! use culpa::throws;
//!
//! // Set the default error type for this module:
//! use std::io::Error;
//!
//! #[throws]
//! fn print() {
//!    let file = std::fs::read_to_string("my_file.txt")?;
//!    println!("{}", file);
//! }
//! ```
//!
//! # Throwing as an Option
//!
//! This syntax can also support functions which return an `Option` instead of a `Result`. To use
//! this with `throws` pass `as Option` as the argument in place of the error type, to use it with
//! `try_fn` just put it as the return type like normal
//!
//! In functions that return `Option`, you can use the `throw!()` macro without any argument to
//! return `None`.
//!
//! ## Example
//!
//! ```
//! #[culpa::throws(as Option)]
//! fn example<T: Eq + Ord>(slice: &[T], needle: &T) -> usize {
//!     if !slice.contains(needle) {
//!         culpa::throw!();
//!     }
//!     slice.binary_search(needle).ok()?
//! }
//!
//! #[culpa::try_fn]
//! fn example_as_try_fn<T: Eq + Ord>(slice: &[T], needle: &T) -> Option<usize> {
//!     if !slice.contains(needle) {
//!         culpa::throw!();
//!     }
//!     slice.binary_search(needle).ok()?
//! }
//! ```
//!
//! # Other `Try` types
//!
//! The `?` syntax in Rust is controlled by a trait called `Try`, which is currently unstable.
//! Because this feature is unstable and I don't want to maintain compatibility if its interface
//! changes, this crate currently only works with two stable `Try` types: `Result` and `Option`.
//! However, its designed so that it will hopefully support other `Try` types as well in the
//! future.
//!
//! It's worth noting that `Try` also has some other stable implementations: specifically `Poll`.
//! Because of the somewhat unusual implementation of `Try` for those types, this crate does not
//! support `throws` syntax on functions that return `Poll` (so you can't use this syntax when
//! implementing a `Future` by hand, for example). I hope to come up with a way to support `Poll`
//! in the future.

#[doc(inline)]
/// Annotates a function that "throws" a Result.
///
/// See the main crate docs for more details.
pub use culpa_macros::throws;

#[doc(inline)]
/// Annotates a function that implicitly wraps a try block.
///
/// See the main crate docs for more details.
pub use culpa_macros::try_fn;

/// Throw an error.
///
/// This macro is equivalent to `Err($err)?`.
#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        return <_ as $crate::__internal::_Throw>::from_error((::core::convert::From::from($err)))
    };
    () => {
        return <_ as ::core::default::Default>::default()
    };
}

#[doc(hidden)]
pub mod __internal {
    pub trait _Throw {
        type Error;
        fn from_error(error: Self::Error) -> Self;
    }

    mod stable {
        use core::task::Poll;

        impl<T, E> super::_Throw for Result<T, E> {
            type Error = E;
            fn from_error(error: Self::Error) -> Self {
                Err(error)
            }
        }

        impl<T, E> super::_Throw for Poll<Result<T, E>> {
            type Error = E;

            fn from_error(error: Self::Error) -> Self {
                Poll::Ready(Err(error))
            }
        }

        impl<T, E> super::_Throw for Poll<Option<Result<T, E>>> {
            type Error = E;

            fn from_error(error: Self::Error) -> Self {
                Poll::Ready(Some(Err(error)))
            }
        }
    }
}

/// Test that function is marked as dead code, because compile_fail is such a blunt instrument
/// these tests should be kept in sync in pairs other than the deny, to ensure there's not another
/// reason to fail compilation
///
/// ```
/// #[culpa::throws(())]
/// fn f() {}
/// ```
/// ```compile_fail
/// #[deny(dead_code)]
/// #[culpa::throws(())]
/// fn f() {}
/// ```
///
/// ```
/// pub struct Foo;
/// impl Foo {
///   #[culpa::throws(())]
///   fn f() {}
/// }
/// ```
/// ```compile_fail
/// pub struct Foo;
/// impl Foo {
///   #[deny(dead_code)]
///   #[culpa::throws(())]
///   fn f() {}
/// }
/// ```
const _DEAD_CODE: () = ();

/// ```compile_fail
/// #[culpa::try_(())]
/// fn f() {}
/// ```
const _NO_TRY_ARGS: () = ();
