#![allow(unused_braces)]
#![allow(clippy::redundant_closure_call)]

use culpa::{throw, try_expr};

#[test]
#[rustfmt::skip]
fn unit() {
    let ok = Result::<(), ()>::Ok(());
    assert_eq!(ok, try_expr!(|| {})());
    assert_eq!(ok, try_expr!(|| ())());
    assert_eq!(ok, try_expr!(|| -> Result<(), ()> {})());
    assert_eq!(ok, try_expr!(|| { return; })());
    assert_eq!(ok, try_expr!(|| { return (); })());
    assert_eq!(ok, try_expr!(|| -> Result<(), ()> { return; })());
    assert_eq!(ok, try_expr!(|| -> Result<(), ()> { return (); })());
}

#[test]
#[rustfmt::skip]
fn integer() {
    let ok = Result::<i32, ()>::Ok(1);
    assert_eq!(ok, try_expr!(|| { 1 })());
    assert_eq!(ok, try_expr!(|| 1)());
    assert_eq!(ok, try_expr!(|| -> Result<i32, ()> { 1 })());
    assert_eq!(ok, try_expr!(|| { return 1; })());
    assert_eq!(ok, try_expr!(|| -> Result<i32, ()> { return 1; })());
    assert_eq!(ok, try_expr!(|| -> _ { 1 })());
}

#[test]
#[rustfmt::skip]
fn throws_unit() {
    let err = Result::<(), ()>::Err(());
    assert_eq!(err, try_expr!(|| { throw!(()) })());
    assert_eq!(err, try_expr!(|| throw!(()))());
    assert_eq!(err, try_expr!(|| -> Result<(), ()> { throw!(()) })());
}

#[test]
#[rustfmt::skip]
fn throws_integer() {
    let err = Result::<(), i32>::Err(1);
    assert_eq!(err, try_expr!(|| { throw!(1)} )());
    assert_eq!(err, try_expr!(|| throw!(1))());
    assert_eq!(err, try_expr!(|| -> Result<(), i32> { throw!(1) })());
}

#[test]
fn has_inner_fn() {
    assert_eq!(
        Result::<(), ()>::Ok(()),
        try_expr!(|| {
            fn foo() -> i32 {
                5
            }
            assert_eq!(5, foo());
        })(),
    );
}

#[test]
fn has_inner_closure() {
    assert_eq!(
        Result::<(), ()>::Ok(()),
        try_expr!(|| {
            assert_eq!(5, (|| 5)());
        })(),
    );
}

#[test]
#[rustfmt::skip]
fn option_unit() {
    let some = Option::<()>::Some(());
    assert_eq!(some, try_expr!(|| {})());
    assert_eq!(some, try_expr!(|| ())());
    assert_eq!(some, try_expr!(|| -> Option<()> {})());
    assert_eq!(some, try_expr!(|| { return; })());
    assert_eq!(some, try_expr!(|| { return (); })());
    assert_eq!(some, try_expr!(|| -> Option<()> { return; })());
    assert_eq!(some, try_expr!(|| -> Option<()> { return (); })());
}

#[test]
#[rustfmt::skip]
fn option_integer() {
    let some = Option::<i32>::Some(1);
    assert_eq!(some, try_expr!(|| { 1 })());
    assert_eq!(some, try_expr!(|| 1)());
    assert_eq!(some, try_expr!(|| -> Option<i32> { 1 })());
    assert_eq!(some, try_expr!(|| { return 1; })());
    assert_eq!(some, try_expr!(|| -> Option<i32> { return 1; })());
    assert_eq!(some, try_expr!(|| -> _ { 1 })());
}

#[test]
#[rustfmt::skip]
fn option_throws() {
    let none = Option::<()>::None;
    assert_eq!(none, try_expr!(|| { throw!() })());
    assert_eq!(none, try_expr!(|| throw!())());
    assert_eq!(none, try_expr!(|| -> Option<()> { throw!() })());
}
