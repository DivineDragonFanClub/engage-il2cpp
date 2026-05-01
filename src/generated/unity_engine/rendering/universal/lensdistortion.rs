
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::volumecomponent::IVolumeComponent;
use crate::unity_engine::rendering::volumecomponent::VolumeComponent;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/lensdistortion/LensDistortion.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "LensDistortion")]
#[parent(crate::unity_engine::rendering::volumecomponent::VolumeComponent)]
pub struct LensDistortion {
    #[rename(name = "intensity")]
    pub intensity: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "xMultiplier")]
    pub x_multiplier: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "yMultiplier")]
    pub y_multiplier: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "center")]
    pub center: crate::unity_engine::rendering::vector2parameter::Vector2Parameter,
    #[rename(name = "scale")]
    pub scale: crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
}

#[cfg(feature = "unity_engine-rendering-universal-lensdistortion")]
#[::unity2::methods]
impl LensDistortion {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-lensdistortion")]
impl LensDistortion {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LensDistortion),
                ::core::stringify!(new),
            )
        });
        <Self as ILensDistortionMethods>::ctor(this);
        this
    }
}
