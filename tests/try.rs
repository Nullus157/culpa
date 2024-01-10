use culpa::{throw, try_fn};

type Error = isize;

#[try_fn]
pub fn unit_fn() -> Result<(), Error> {}

#[try_fn]
pub fn returns_fn() -> Result<i32, Error> {
    return 0;
}

#[try_fn]
pub fn returns_unit_fn() -> Result<(), Error> {
    if true {
        return;
    }
}

#[try_fn]
pub fn tail_returns_value() -> Result<i32, Error> {
    0
}

#[try_fn]
pub async fn async_fn() -> Result<(), Error> {}

#[try_fn]
pub async fn async_fn_with_ret() -> Result<i32, Error> {
    0
}

#[try_fn]
pub fn throws_error() -> Result<(), i32> {
    if true {
        throw!(0);
    }
}

#[try_fn]
pub fn throws_and_has_return_type() -> Result<&'static str, i32> {
    if true {
        return "success";
    } else if false {
        throw!(0);
    }
    "okay"
}

#[try_fn]
pub fn throws_generics<E>() -> Result<(), E> {}

pub struct Foo;

impl Foo {
    #[try_fn]
    pub fn static_method() -> Result<(), Error> {}

    #[try_fn]
    pub fn bar(&self) -> Result<i32, Error> {
        if true {
            return 1;
        }
        0
    }
}

#[try_fn]
pub fn has_inner_fn() -> Result<(), Error> {
    fn inner_fn() -> i32 {
        0
    }
    let _: i32 = inner_fn();
}

#[try_fn]
pub fn has_inner_closure() -> Result<(), Error> {
    let f = || 0;
    let _: i32 = f();
}

#[try_fn]
pub async fn has_inner_async_block() -> Result<(), Error> {
    let f = async { 0 };
    let _: i32 = f.await;
}

#[try_fn]
pub fn throws_as_result() -> Result<i32, Error> {
    0
}

#[try_fn]
pub fn throws_as_result_alias() -> std::io::Result<i32> {
    0
}

#[try_fn]
pub fn ommitted_error() -> Result<(), Error> {}

pub mod foo {
    use culpa::{throw, try_fn};

    pub type Error = i32;

    #[try_fn]
    pub fn throws_integer() -> Result<(), i32> {
        throw!(0);
    }
}

pub mod foo_trait_obj {
    use culpa::try_fn;
    pub trait FooTrait {}

    struct FooStruct;

    pub struct FooError;
    impl FooTrait for FooStruct {}

    #[try_fn]
    pub fn foo() -> Result<Box<dyn FooTrait>, FooError> {
        Box::new(FooStruct)
    }
}

#[try_fn]
pub fn let_else(a: Option<u8>) -> Result<u8, Error> {
    let Some(a) = a else {
        return 0;
    };
    a
}

#[try_fn]
pub fn impl_trait() -> Result<impl std::fmt::Debug, Error> {}

#[try_fn]
#[deny(unreachable_code)]
pub fn unreachable() -> Result<(), i32> {
    todo!()
}

trait Example {
    #[try_fn]
    fn foo() -> Result<i32, Error>;
}

#[try_fn]
fn as_option(x: bool) -> Option<i32> {
    if x {
        throw!();
    }
    0
}

#[test]
fn test_as_option_true() {
    assert_eq!(None, as_option(true));
}

#[test]
fn test_as_option_false() {
    assert_eq!(Some(0), as_option(false))
}
