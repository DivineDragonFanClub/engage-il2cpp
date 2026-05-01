
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharpmoduleattribute/MoonSharpModuleAttribute.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "MoonSharpModuleAttribute")]
pub struct MoonSharpModuleAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmoduleattribute")]
#[::unity2::methods]
impl MoonSharpModuleAttribute {
    #[method(name = "get_Namespace", args = 0)]
    pub fn get_namespace(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Namespace", args = 1)]
    pub fn set_namespace(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharpmoduleattribute")]
impl MoonSharpModuleAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpModuleAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpModuleAttributeMethods>::ctor(this);
        this
    }
}
