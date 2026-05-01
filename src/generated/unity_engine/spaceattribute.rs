
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::propertyattribute::IPropertyAttribute;
use crate::unity_engine::propertyattribute::PropertyAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/spaceattribute/SpaceAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SpaceAttribute")]
#[parent(crate::unity_engine::propertyattribute::PropertyAttribute)]
pub struct SpaceAttribute {
    #[rename(name = "height")]
    pub height: f32,
}

#[cfg(feature = "unity_engine-spaceattribute")]
#[::unity2::methods]
impl SpaceAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, height: f32) -> ();
}

#[cfg(feature = "unity_engine-spaceattribute")]
impl SpaceAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SpaceAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ISpaceAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(height: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SpaceAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISpaceAttributeMethods>::ctor_2(this, height);
        this
    }
}
