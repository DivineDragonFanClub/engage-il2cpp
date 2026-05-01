
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::propertyattribute::IPropertyAttribute;
use crate::unity_engine::propertyattribute::PropertyAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/minattribute/MinAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "MinAttribute")]
#[parent(crate::unity_engine::propertyattribute::PropertyAttribute)]
pub struct MinAttribute {
    #[rename(name = "min")]
    pub min: f32,
}

#[cfg(feature = "unity_engine-minattribute")]
#[::unity2::methods]
impl MinAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, min: f32) -> ();
}

#[cfg(feature = "unity_engine-minattribute")]
impl MinAttribute {
    pub fn new(min: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MinAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IMinAttributeMethods>::ctor(this, min);
        this
    }
}
