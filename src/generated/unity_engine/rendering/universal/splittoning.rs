
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::volumecomponent::IVolumeComponent;
use crate::unity_engine::rendering::volumecomponent::VolumeComponent;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/splittoning/SplitToning.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "SplitToning")]
#[parent(crate::unity_engine::rendering::volumecomponent::VolumeComponent)]
pub struct SplitToning {
    #[rename(name = "shadows")]
    pub shadows: crate::unity_engine::rendering::colorparameter::ColorParameter,
    #[rename(name = "highlights")]
    pub highlights: crate::unity_engine::rendering::colorparameter::ColorParameter,
    #[rename(name = "balance")]
    pub balance: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
}

#[cfg(feature = "unity_engine-rendering-universal-splittoning")]
#[::unity2::methods]
impl SplitToning {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-splittoning")]
impl SplitToning {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SplitToning),
                ::core::stringify!(new),
            )
        });
        <Self as ISplitToningMethods>::ctor(this);
        this
    }
}
