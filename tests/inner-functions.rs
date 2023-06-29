use culpa::throws;

pub type Error = isize;

#[throws]
pub fn inner_function() {
    fn foo() {
    }
    foo();
}

#[throws]
pub fn fn_parameters(_: fn()) {
}

#[throws]
pub fn fn_type_alias() {
    #[allow(dead_code)]
    type X = fn();
}

#[throws]
pub fn type_ascription() {
    let _: fn() = panic!();
}

#[throws(std::io::Error)]
pub fn dyn_fn_once() {
    let mut _unused: Box<dyn FnOnce()> = Box::new(||());
}
