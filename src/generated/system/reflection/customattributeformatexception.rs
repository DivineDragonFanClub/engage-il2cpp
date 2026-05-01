
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/customattributeformatexception/CustomAttributeFormatException.md")))]
#[::unity2::class(
    namespace = "System.Reflection",
    name = "CustomAttributeFormatException"
)]
pub struct CustomAttributeFormatException {}

#[cfg(feature = "system-reflection-customattributeformatexception")]
#[::unity2::methods]
impl CustomAttributeFormatException {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, message: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "system-reflection-customattributeformatexception")]
impl CustomAttributeFormatException {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomAttributeFormatException),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomAttributeFormatExceptionMethods>::ctor(this);
        this
    }

    pub fn new_2(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomAttributeFormatException),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICustomAttributeFormatExceptionMethods>::ctor_2(this, message);
        this
    }
}
