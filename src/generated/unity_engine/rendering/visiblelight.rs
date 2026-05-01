
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/visiblelight/VisibleLight.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct VisibleLight {
    pub m_light_type: crate::unity_engine::lighttype::LightType,
    pub m_final_color: crate::unity_engine::color::Color,
    pub m_screen_rect: crate::unity_engine::rect::Rect,
    pub m_local_to_world_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    pub m_range: f32,
    pub m_spot_angle: f32,
    pub m_instance_id: i32,
    pub m_flags: crate::unity_engine::rendering::visiblelightflags::VisibleLightFlags,
}

impl ::unity2::ClassIdentity for VisibleLight {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "VisibleLight";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VisibleLight {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-visiblelight")]
#[::unity2::methods(value)]
impl VisibleLight {
    #[method(name = "get_light", args = 0)]
    pub fn get_light(self) -> crate::unity_engine::light::Light;

    #[method(name = "get_lightType", args = 0)]
    pub fn get_light_type(self) -> crate::unity_engine::lighttype::LightType;

    #[method(name = "get_finalColor", args = 0)]
    pub fn get_final_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "get_localToWorldMatrix", args = 0)]
    pub fn get_local_to_world_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_range", args = 0)]
    pub fn get_range(self) -> f32;

    #[method(name = "get_spotAngle", args = 0)]
    pub fn get_spot_angle(self) -> f32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::rendering::visiblelight::VisibleLight) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
