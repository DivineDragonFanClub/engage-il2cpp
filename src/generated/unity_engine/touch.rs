
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/touch/Touch.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Touch {
    pub m_finger_id: i32,
    pub m_position: crate::unity_engine::vector2::Vector2,
    pub m_raw_position: crate::unity_engine::vector2::Vector2,
    pub m_position_delta: crate::unity_engine::vector2::Vector2,
    pub m_time_delta: f32,
    pub m_tap_count: i32,
    pub m_phase: crate::unity_engine::touchphase::TouchPhase,
    pub m_type: crate::unity_engine::touchtype::TouchType,
    pub m_pressure: f32,
    pub m_maximum_possible_pressure: f32,
    pub m_radius: f32,
    pub m_radius_variance: f32,
    pub m_altitude_angle: f32,
    pub m_azimuth_angle: f32,
}

impl ::unity2::ClassIdentity for Touch {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Touch";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Touch {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-touch")]
#[::unity2::methods(value)]
impl Touch {
    #[method(name = "get_fingerId", args = 0)]
    pub fn get_finger_id(self) -> i32;

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_deltaPosition", args = 0)]
    pub fn get_delta_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_phase", args = 0)]
    pub fn get_phase(self) -> crate::unity_engine::touchphase::TouchPhase;

    #[method(name = "get_type", args = 0)]
    pub fn get_type(self) -> crate::unity_engine::touchtype::TouchType;
}
