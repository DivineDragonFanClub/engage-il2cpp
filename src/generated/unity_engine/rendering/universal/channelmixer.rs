
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::volumecomponent::IVolumeComponent;
use crate::unity_engine::rendering::volumecomponent::VolumeComponent;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/channelmixer/ChannelMixer.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "ChannelMixer")]
#[parent(crate::unity_engine::rendering::volumecomponent::VolumeComponent)]
pub struct ChannelMixer {
    #[rename(name = "redOutRedIn")]
    pub red_out_red_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "redOutGreenIn")]
    pub red_out_green_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "redOutBlueIn")]
    pub red_out_blue_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "greenOutRedIn")]
    pub green_out_red_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "greenOutGreenIn")]
    pub green_out_green_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "greenOutBlueIn")]
    pub green_out_blue_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "blueOutRedIn")]
    pub blue_out_red_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "blueOutGreenIn")]
    pub blue_out_green_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
    #[rename(name = "blueOutBlueIn")]
    pub blue_out_blue_in:
        crate::unity_engine::rendering::clampedfloatparameter::ClampedFloatParameter,
}

#[cfg(feature = "unity_engine-rendering-universal-channelmixer")]
#[::unity2::methods]
impl ChannelMixer {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-channelmixer")]
impl ChannelMixer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChannelMixer),
                ::core::stringify!(new),
            )
        });
        <Self as IChannelMixerMethods>::ctor(this);
        this
    }
}
