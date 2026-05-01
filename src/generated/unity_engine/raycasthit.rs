
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/raycasthit/RaycastHit.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RaycastHit {
    pub m_point: crate::unity_engine::vector3::Vector3,
    pub m_normal: crate::unity_engine::vector3::Vector3,
    pub m_face_id: u32,
    pub m_distance: f32,
    pub m_uv: crate::unity_engine::vector2::Vector2,
    pub m_collider: i32,
}

impl ::unity2::ClassIdentity for RaycastHit {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "RaycastHit";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RaycastHit {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-raycasthit")]
#[::unity2::methods(value)]
impl RaycastHit {
    #[method(name = "get_collider", args = 0)]
    pub fn get_collider(self) -> crate::unity_engine::collider::Collider;

    #[method(name = "get_point", args = 0)]
    pub fn get_point(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_point", args = 1)]
    pub fn set_point(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_normal", args = 0)]
    pub fn get_normal(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_normal", args = 1)]
    pub fn set_normal(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_triangleIndex", args = 0)]
    pub fn get_triangle_index(self) -> i32;

    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_rigidbody", args = 0)]
    pub fn get_rigidbody(self) -> crate::unity_engine::rigidbody::Rigidbody;
}
