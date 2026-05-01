
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharpmoduleconstantattribute/MoonSharpModuleConstantAttribute.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "MoonSharpModuleConstantAttribute"
)]
pub struct MoonSharpModuleConstantAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmoduleconstantattribute")]
#[::unity2::methods]
impl MoonSharpModuleConstantAttribute {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmoduleconstantattribute")]
impl MoonSharpModuleConstantAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpModuleConstantAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpModuleConstantAttributeMethods>::ctor(this);
        this
    }
}
