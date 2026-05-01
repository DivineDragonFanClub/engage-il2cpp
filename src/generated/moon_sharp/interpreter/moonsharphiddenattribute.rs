
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/moonsharphiddenattribute/MoonSharpHiddenAttribute.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "MoonSharpHiddenAttribute")]
pub struct MoonSharpHiddenAttribute {}

#[cfg(feature = "moon_sharp-interpreter-moonsharphiddenattribute")]
#[::unity2::methods]
impl MoonSharpHiddenAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-moonsharphiddenattribute")]
impl MoonSharpHiddenAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoonSharpHiddenAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMoonSharpHiddenAttributeMethods>::ctor(this);
        this
    }
}
