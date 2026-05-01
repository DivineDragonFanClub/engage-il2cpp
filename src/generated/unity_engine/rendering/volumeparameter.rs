
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumeparameter/VolumeParameter.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "VolumeParameter")]
#[parent(crate::system::object::Object)]
pub struct VolumeParameter {
    #[static_field]
    #[rename(name = "k_DebuggerDisplay")]
    pub k_debugger_display: ::unity2::Il2CppString,
    #[rename(name = "m_OverrideState")]
    pub m_override_state: bool,
}

#[cfg(feature = "unity_engine-rendering-volumeparameter")]
#[::unity2::methods]
impl VolumeParameter {
    #[method(name = "get_overrideState", args = 0)]
    pub fn get_override_state(self) -> bool;

    #[method(name = "set_overrideState", args = 1)]
    pub fn set_override_state(self, value: bool) -> ();

    #[method(name = "Interp", args = 3)]
    pub fn interp(
        self,
        from: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        to: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
        t: f32,
    ) -> ();

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(
        self,
        parameter: crate::unity_engine::rendering::volumeparameter::VolumeParameter,
    ) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "IsObjectParameter", args = 1)]
    pub fn is_object_parameter(r#type: ::unity2::SystemType) -> bool;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-volumeparameter")]
impl VolumeParameter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeParameterMethods>::ctor(this);
        this
    }
}
