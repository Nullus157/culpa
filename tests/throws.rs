use fehler::*;

type Error = ();

#[throws(_)]
pub fn unit_fn() { }

#[throws(_)]
pub fn returns_fn() -> i32 {
    return 0;
}

#[throws(_)]
pub fn returns_unit_fn() {
    if true { return; }
}

#[throws(_)]
pub fn explicit_unit() -> () { }

#[throws(_)]
pub fn tail_returns_value() -> i32 {
    0
}

#[throws(_)]
pub async fn async_fn() { }

#[throws(_)]
pub async fn async_fn_with_ret() -> i32 {
    0
}

#[throws(i32)]
pub fn throws_error() {
    if true { throw!(0); }
}

#[throws(i32)]
pub fn throws_and_has_return_type() -> &'static str {
    if true {
        return "success";
    } else if false {
        throw!(0);
    }
    "okay"
}

#[throws(E)]
pub fn throws_generics<E>() { }

pub struct Foo;

impl Foo {
    #[throws(_)]
    pub fn static_method() { }

    #[throws(_)]
    pub fn bar(&self) -> i32 { if true { return 1; } 0 }
}


#[throws(_)]
pub fn has_inner_fn() {
    fn inner_fn() -> i32 { 0 }
    let _: i32 = inner_fn();
}

#[throws(_)]
pub fn has_inner_closure() {
    let f = || 0;
    let _: i32 = f();
}

#[throws(_)]
pub async fn has_inner_async_block() {
    let f = async { 0 };
    let _: i32 = f.await;
}

#[throws(_ as Result)]
pub fn throws_as_result() -> i32 {
    0
}

#[throws(as std::io::Result)]
pub fn throws_as_result_alias() -> i32 {
    0
}

#[throws]
pub fn ommitted_error() { }

pub mod foo {
    use fehler::*;

    pub type Error = i32;

    #[throws]
    pub fn throws_integer() {
        throw!(0);
    }
}

pub mod foo_trait_obj {
    use fehler::throws;
    pub trait FooTrait {}

    struct FooStruct;

    pub struct FooError;
    impl FooTrait for FooStruct {}

    #[throws(FooError)]
    pub fn foo() -> Box<dyn FooTrait> {
        Box::new(FooStruct)
    }
}

#[throws]
pub fn let_else(a: Option<u8>) -> u8 {
    let Some(a) = a else {
        return 0;
    };
    a
}

#[throws(i32)]
#[deny(unreachable_code)]
pub fn unreachable() {
    todo!()
}