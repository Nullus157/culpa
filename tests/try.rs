use culpa::{throw, try_};

type Error = isize;

#[try_]
pub fn unit_fn() -> Result<(), Error> {}

#[try_]
pub fn returns_fn() -> Result<i32, Error> {
    return 0;
}

#[try_]
pub fn returns_unit_fn() -> Result<(), Error> {
    if true {
        return;
    }
}

#[try_]
pub fn tail_returns_value() -> Result<i32, Error> {
    0
}

#[try_]
pub async fn async_fn() -> Result<(), Error> {}

#[try_]
pub async fn async_fn_with_ret() -> Result<i32, Error> {
    0
}

#[try_]
pub fn throws_error() -> Result<(), i32> {
    if true {
        throw!(0);
    }
}

#[try_]
pub fn throws_and_has_return_type() -> Result<&'static str, i32> {
    if true {
        return "success";
    } else if false {
        throw!(0);
    }
    "okay"
}

#[try_]
pub fn throws_generics<E>() -> Result<(), E> {}

pub struct Foo;

impl Foo {
    #[try_]
    pub fn static_method() -> Result<(), Error> {}

    #[try_]
    pub fn bar(&self) -> Result<i32, Error> {
        if true {
            return 1;
        }
        0
    }
}

#[try_]
pub fn has_inner_fn() -> Result<(), Error> {
    fn inner_fn() -> i32 {
        0
    }
    let _: i32 = inner_fn();
}

#[try_]
pub fn has_inner_closure() -> Result<(), Error> {
    let f = || 0;
    let _: i32 = f();
}

#[try_]
pub async fn has_inner_async_block() -> Result<(), Error> {
    let f = async { 0 };
    let _: i32 = f.await;
}

#[try_]
pub fn throws_as_result() -> Result<i32, Error> {
    0
}

#[try_]
pub fn throws_as_result_alias() -> std::io::Result<i32> {
    0
}

#[try_]
pub fn ommitted_error() -> Result<(), Error> {}

pub mod foo {
    use culpa::{throw, try_};

    pub type Error = i32;

    #[try_]
    pub fn throws_integer() -> Result<(), i32> {
        throw!(0);
    }
}

pub mod foo_trait_obj {
    use culpa::try_;
    pub trait FooTrait {}

    struct FooStruct;

    pub struct FooError;
    impl FooTrait for FooStruct {}

    #[try_]
    pub fn foo() -> Result<Box<dyn FooTrait>, FooError> {
        Box::new(FooStruct)
    }
}

#[try_]
pub fn let_else(a: Option<u8>) -> Result<u8, Error> {
    let Some(a) = a else {
        return 0;
    };
    a
}

#[try_]
pub fn impl_trait() -> Result<impl std::fmt::Debug, Error> {}

#[try_]
#[deny(unreachable_code)]
pub fn unreachable() -> Result<(), i32> {
    todo!()
}

trait Example {
    #[try_]
    fn foo() -> Result<i32, Error>;
}

#[try_]
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
