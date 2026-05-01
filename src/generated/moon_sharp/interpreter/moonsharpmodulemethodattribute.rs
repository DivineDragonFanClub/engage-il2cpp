
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharpmodulemethodattribute/MoonSharpModuleMethodAttribute.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "MoonSharpModuleMethodAttribute"
)]
pub struct MoonSharpModuleMethodAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmodulemethodattribute")]
#[::unity2::methods]
impl MoonSharpModuleMethodAttribute {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmodulemethodattribute")]
impl MoonSharpModuleMethodAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpModuleMethodAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpModuleMethodAttributeMethods>::ctor(this);
        this
    }
}
