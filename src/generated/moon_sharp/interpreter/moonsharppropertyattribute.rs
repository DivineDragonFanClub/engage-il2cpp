
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharppropertyattribute/MoonSharpPropertyAttribute.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "MoonSharpPropertyAttribute"
)]
pub struct MoonSharpPropertyAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharppropertyattribute")]
#[::unity2::methods]
impl MoonSharpPropertyAttribute {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharppropertyattribute")]
impl MoonSharpPropertyAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpPropertyAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpPropertyAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpPropertyAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMoonSharpPropertyAttributeMethods>::ctor_2(this, name);
        this
    }
}
