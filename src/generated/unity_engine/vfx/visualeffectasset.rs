
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::vfx::visualeffectobject::IVisualEffectObject;
use crate::unity_engine::vfx::visualeffectobject::VisualEffectObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vfx/visualeffectasset/VisualEffectAsset.md")))]
#[::unity2::class(namespace = "UnityEngine.VFX", name = "VisualEffectAsset")]
#[parent(crate::unity_engine::vfx::visualeffectobject::VisualEffectObject)]
pub struct VisualEffectAsset {
    #[static_field]
    #[rename(name = "PlayEventID")]
    pub play_event_id: i32,
    #[static_field]
    #[rename(name = "StopEventID")]
    pub stop_event_id: i32,
}

#[cfg(feature = "unity_engine-vfx-visualeffectasset")]
#[::unity2::methods]
impl VisualEffectAsset {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-vfx-visualeffectasset")]
impl VisualEffectAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VisualEffectAsset),
                ::core::stringify!(new),
            )
        });
        <Self as IVisualEffectAssetMethods>::ctor(this);
        this
    }
}
