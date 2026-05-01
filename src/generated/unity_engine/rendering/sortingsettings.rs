
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/sortingsettings/SortingSettings.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SortingSettings {
    pub m_world_to_camera_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    pub m_camera_position: crate::unity_engine::vector3::Vector3,
    pub m_custom_axis: crate::unity_engine::vector3::Vector3,
    pub m_criteria: crate::unity_engine::rendering::sortingcriteria::SortingCriteria,
    pub m_distance_metric: crate::unity_engine::rendering::distancemetric::DistanceMetric,
    pub m_previous_vp_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    pub m_non_jittered_vp_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
}

impl ::unity2::ClassIdentity for SortingSettings {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SortingSettings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortingSettings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-sortingsettings")]
#[::unity2::methods(value)]
impl SortingSettings {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "set_customAxis", args = 1)]
    pub fn set_custom_axis(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_criteria", args = 0)]
    pub fn get_criteria(self) -> crate::unity_engine::rendering::sortingcriteria::SortingCriteria;

    #[method(name = "set_criteria", args = 1)]
    pub fn set_criteria(
        self,
        value: crate::unity_engine::rendering::sortingcriteria::SortingCriteria,
    ) -> ();

    #[method(name = "set_distanceMetric", args = 1)]
    pub fn set_distance_metric(
        self,
        value: crate::unity_engine::rendering::distancemetric::DistanceMetric,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::sortingsettings::SortingSettings,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
