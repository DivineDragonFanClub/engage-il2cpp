
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::volumecomponent::IVolumeComponent;
use crate::unity_engine::rendering::volumecomponent::VolumeComponent;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/tonemapping/Tonemapping.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "Tonemapping")]
#[parent(crate::unity_engine::rendering::volumecomponent::VolumeComponent)]
pub struct Tonemapping {
# [rename (name = "mode")] pub mode : crate :: unity_engine :: rendering :: universal :: tonemappingmodeparameter :: TonemappingModeParameter ,
# [rename (name = "toeStrength")] pub toe_strength : crate :: unity_engine :: rendering :: clampedfloatparameter :: ClampedFloatParameter ,
# [rename (name = "toeLength")] pub toe_length : crate :: unity_engine :: rendering :: clampedfloatparameter :: ClampedFloatParameter ,
# [rename (name = "shoulderStrength")] pub shoulder_strength : crate :: unity_engine :: rendering :: clampedfloatparameter :: ClampedFloatParameter ,
# [rename (name = "shoulderLength")] pub shoulder_length : crate :: unity_engine :: rendering :: minfloatparameter :: MinFloatParameter ,
# [rename (name = "shoulderAngle")] pub shoulder_angle : crate :: unity_engine :: rendering :: clampedfloatparameter :: ClampedFloatParameter ,
# [rename (name = "gamma")] pub gamma : crate :: unity_engine :: rendering :: minfloatparameter :: MinFloatParameter ,
}

#[cfg(feature = "unity_engine-rendering-universal-tonemapping")]
#[::unity2::methods]
impl Tonemapping {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-tonemapping")]
impl Tonemapping {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Tonemapping),
                ::core::stringify!(new),
            )
        });
        <Self as ITonemappingMethods>::ctor(this);
        this
    }
}
