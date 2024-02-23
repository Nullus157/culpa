#![allow(unused_braces)]
#![allow(clippy::redundant_closure_call)]

use culpa::{throw, throws_expr};

#[test]
#[rustfmt::skip]
fn unit() {
    type Error = ();
    let ok = Result::<(), ()>::Ok(());
    assert_eq!(ok, throws_expr!(|| {})());
    assert_eq!(ok, throws_expr!(|| ())());
    assert_eq!(ok, throws_expr!(|| -> () {})());
    assert_eq!(ok, throws_expr!(|| { return; })());
    assert_eq!(ok, throws_expr!(|| { return (); })());
    assert_eq!(ok, throws_expr!(|| -> () { return; })());
    assert_eq!(ok, throws_expr!(|| -> () { return (); })());
}

#[test]
#[rustfmt::skip]
fn integer() {
    type Error = ();
    let ok = Result::<i32, ()>::Ok(1);
    assert_eq!(ok, throws_expr!(|| { 1 })());
    assert_eq!(ok, throws_expr!(|| 1)());
    assert_eq!(ok, throws_expr!(|| -> i32 { 1 })());
    assert_eq!(ok, throws_expr!(|| { return 1; })());
    assert_eq!(ok, throws_expr!(|| -> i32 { return 1; })());
    assert_eq!(ok, throws_expr!(|| -> _ { 1 })());
}

#[test]
#[rustfmt::skip]
fn throws_unit() {
    type Error = ();
    let err = Result::<(), ()>::Err(());
    assert_eq!(err, throws_expr!(|| { throw!(()) })());
    assert_eq!(err, throws_expr!(|| throw!(()))());
    assert_eq!(err, throws_expr!(|| -> () { throw!(()) })());
}

#[test]
#[rustfmt::skip]
fn throws_integer() {
    type Error = i32;
    let err = Result::<(), i32>::Err(1);
    assert_eq!(err, throws_expr!(|| { throw!(1)} )());
    assert_eq!(err, throws_expr!(|| throw!(1))());
    assert_eq!(err, throws_expr!(|| -> () { throw!(1) })());
}

#[test]
fn has_inner_fn() {
    type Error = ();
    assert_eq!(
        Result::<(), ()>::Ok(()),
        throws_expr!(|| {
            fn foo() -> i32 {
                5
            }
            assert_eq!(5, foo());
        })(),
    );
}

#[test]
fn has_inner_closure() {
    type Error = ();
    assert_eq!(
        Result::<(), ()>::Ok(()),
        throws_expr!(|| {
            assert_eq!(5, (|| 5)());
        })(),
    );
}
