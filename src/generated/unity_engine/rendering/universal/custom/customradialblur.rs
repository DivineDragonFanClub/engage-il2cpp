
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::volumecomponent::IVolumeComponent;
use crate::unity_engine::rendering::volumecomponent::VolumeComponent;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/customradialblur/CustomRadialBlur.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom",
    name = "CustomRadialBlur"
)]
#[parent(crate::unity_engine::rendering::volumecomponent::VolumeComponent)]
pub struct CustomRadialBlur {
    #[rename(name = "intensity")]
    pub intensity: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "centerU")]
    pub center_u: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "centerV")]
    pub center_v: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "beginAlpha")]
    pub begin_alpha: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "endAlpha")]
    pub end_alpha: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "sampleCount")]
    pub sample_count: crate::unity_engine::rendering::clampedintparameter::ClampedIntParameter,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-customradialblur")]
#[::unity2::methods]
impl CustomRadialBlur {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-customradialblur")]
impl CustomRadialBlur {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomRadialBlur),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomRadialBlurMethods>::ctor(this);
        this
    }
}
