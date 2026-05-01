
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::propertyattribute::IPropertyAttribute;
use crate::unity_engine::propertyattribute::PropertyAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/multilineattribute/MultilineAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "MultilineAttribute")]
#[parent(crate::unity_engine::propertyattribute::PropertyAttribute)]
pub struct MultilineAttribute {
    #[rename(name = "lines")]
    pub lines: i32,
}

#[cfg(feature = "unity_engine-multilineattribute")]
#[::unity2::methods]
impl MultilineAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-multilineattribute")]
impl MultilineAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MultilineAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMultilineAttributeMethods>::ctor(this);
        this
    }
}
