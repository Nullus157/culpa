use culpa::{throw, throws_expr};

#[test]
#[allow(clippy::unused_unit)]
fn unit() {
    type Error = ();
    let ok = Result::<(), ()>::Ok(());
    assert_eq!(ok, poll(throws_expr!(async {})));
    assert_eq!(ok, poll(throws_expr!(async { () })));
    assert_eq!(
        ok,
        poll(throws_expr!(async {
            return;
        }))
    );
    assert_eq!(
        ok,
        poll(throws_expr!(async {
            return ();
        }))
    );
}

#[test]
fn integer() {
    type Error = ();
    let ok = Result::<i32, ()>::Ok(1);
    assert_eq!(ok, poll(throws_expr!(async { 1 })));
    assert_eq!(
        ok,
        poll(throws_expr!(async {
            return 1;
        }))
    );
}

#[test]
fn throws_unit() {
    type Error = ();
    let err = Result::<(), ()>::Err(());
    assert_eq!(err, poll(throws_expr!(async { throw!(()) })));
}

#[test]
fn throws_integer() {
    type Error = i32;
    let err = Result::<(), i32>::Err(1);
    assert_eq!(err, poll(throws_expr!(async { throw!(1) })));
}

#[test]
fn has_inner_fn() {
    type Error = ();
    assert_eq!(
        Result::<(), ()>::Ok(()),
        poll(throws_expr!(async {
            async fn foo() -> i32 {
                5
            }
            assert_eq!(5, foo().await);
        })),
    );
}

#[test]
fn has_inner_closure() {
    type Error = ();
    assert_eq!(
        Result::<(), ()>::Ok(()),
        poll(throws_expr!(async {
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
