
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/clipperregistry/ClipperRegistry.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ClipperRegistry")]
#[parent(crate::system::object::Object)]
pub struct ClipperRegistry {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::unity_engine::ui::clipperregistry::ClipperRegistry,
    #[rename(name = "m_Clippers")]
    pub m_clippers: crate::unity_engine::ui::collections::indexedset_1::IndexedSet_1<
        crate::unity_engine::ui::iclipper::IClipper,
    >,
}

#[cfg(feature = "unity_engine-ui-clipperregistry")]
#[::unity2::methods]
impl ClipperRegistry {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::unity_engine::ui::clipperregistry::ClipperRegistry;

    #[method(name = "Cull", args = 0)]
    pub fn cull(self) -> ();

    #[method(name = "Register", args = 1)]
    pub fn register(c: crate::unity_engine::ui::iclipper::IClipper) -> ();

    #[method(name = "Unregister", args = 1)]
    pub fn unregister(c: crate::unity_engine::ui::iclipper::IClipper) -> ();
}

#[cfg(feature = "unity_engine-ui-clipperregistry")]
impl ClipperRegistry {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClipperRegistry),
                ::core::stringify!(new),
            )
        });
        <Self as IClipperRegistryMethods>::ctor(this);
        this
    }
}
