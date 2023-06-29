use fehler::throws;

pub type Error = ();

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
