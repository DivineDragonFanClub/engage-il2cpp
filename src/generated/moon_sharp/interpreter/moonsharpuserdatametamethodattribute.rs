
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharpuserdatametamethodattribute/MoonSharpUserDataMetamethodAttribute.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "MoonSharpUserDataMetamethodAttribute"
)]
pub struct MoonSharpUserDataMetamethodAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharpuserdatametamethodattribute")]
#[::unity2::methods]
impl MoonSharpUserDataMetamethodAttribute {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharpuserdatametamethodattribute")]
impl MoonSharpUserDataMetamethodAttribute {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpUserDataMetamethodAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpUserDataMetamethodAttributeMethods>::ctor(this, name);
        this
    }
}
