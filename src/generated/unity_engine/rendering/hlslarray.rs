
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/hlslarray/HLSLArray.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "HLSLArray")]
pub struct HLSLArray {
    #[rename(name = "arraySize")]
    pub array_size: i32,
    #[rename(name = "elementType")]
    pub element_type: ::unity2::SystemType,
}

#[cfg(feature = "unity_engine-rendering-hlslarray")]
#[::unity2::methods]
impl HLSLArray {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, array_size: i32, element_type: ::unity2::SystemType) -> ();
}

#[cfg(feature = "unity_engine-rendering-hlslarray")]
impl HLSLArray {
    pub fn new(array_size: i32, element_type: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HLSLArray),
                ::core::stringify!(new),
            )
        });
        <Self as IHLSLArrayMethods>::ctor(this, array_size, element_type);
        this
    }
}
