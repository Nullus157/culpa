use culpa::{throw, try_expr};

#[test]
#[allow(clippy::unused_unit)]
fn unit() {
    let ok = Result::<(), ()>::Ok(());
    assert_eq!(ok, poll(try_expr!(async {})));
    assert_eq!(ok, poll(try_expr!(async { () })));
    assert_eq!(
        ok,
        poll(try_expr!(async {
            return;
        }))
    );
    assert_eq!(
        ok,
        poll(try_expr!(async {
            return ();
        }))
    );
}

#[test]
fn integer() {
    let ok = Result::<i32, ()>::Ok(1);
    assert_eq!(ok, poll(try_expr!(async { 1 })));
    assert_eq!(
        ok,
        poll(try_expr!(async {
            return 1;
        }))
    );
}

#[test]
fn try_unit() {
    let err = Result::<(), ()>::Err(());
    assert_eq!(err, poll(try_expr!(async { throw!(()) })));
}

#[test]
fn try_integer() {
    let err = Result::<(), i32>::Err(1);
    assert_eq!(err, poll(try_expr!(async { throw!(1) })));
}

#[test]
fn has_inner_fn() {
    assert_eq!(
        Result::<(), ()>::Ok(()),
        poll(try_expr!(async {
            async fn foo() -> i32 {
                5
            }
            assert_eq!(5, foo().await);
        })),
    );
}

#[test]
fn has_inner_closure() {
    assert_eq!(
        Result::<(), ()>::Ok(()),
        poll(try_expr!(async {
            assert_eq!(5, async { 5 }.await);
        })),
    );
}

fn poll<F: std::future::Future>(f: F) -> F::Output {
    struct NoopWake;
    impl std::task::Wake for NoopWake {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let std::task::Poll::Ready(output) = std::pin::pin!(f).poll(
        &mut std::task::Context::from_waker(&std::sync::Arc::new(NoopWake).into()),
    ) else {
        panic!("future was not ready")
    };
    output
}

#[test]
#[allow(clippy::unused_unit)]
fn option_unit() {
    let some = Option::<()>::Some(());
    assert_eq!(some, poll(try_expr!(async {})));
    assert_eq!(some, poll(try_expr!(async { () })));
    assert_eq!(
        some,
        poll(try_expr!(async {
            return;
        }))
    );
    assert_eq!(
        some,
        poll(try_expr!(async {
            return ();
        }))
    );
}

#[test]
fn option_integer() {
    let some = Option::<i32>::Some(1);
    assert_eq!(some, poll(try_expr!(async { 1 })));
    assert_eq!(
        some,
        poll(try_expr!(async {
            return 1;
        }))
    );
}

#[test]
fn option_throws() {
    let none = Option::<()>::None;
    assert_eq!(none, poll(try_expr!(async { throw!() })));
}
