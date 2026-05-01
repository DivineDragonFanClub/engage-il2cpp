
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/displayinfoattribute/DisplayInfoAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DisplayInfoAttribute")]
pub struct DisplayInfoAttribute {
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "order")]
    pub order: i32,
}

#[cfg(feature = "unity_engine-rendering-displayinfoattribute")]
#[::unity2::methods]
impl DisplayInfoAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-displayinfoattribute")]
impl DisplayInfoAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisplayInfoAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDisplayInfoAttributeMethods>::ctor(this);
        this
    }
}
