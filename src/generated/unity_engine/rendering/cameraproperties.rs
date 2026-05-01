
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/cameraproperties/CameraProperties.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CameraProperties {}

impl ::unity2::ClassIdentity for CameraProperties {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "CameraProperties";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CameraProperties {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-cameraproperties")]
#[::unity2::methods(value)]
impl CameraProperties {
    #[method(name = "GetShadowCullingPlane", args = 1)]
    pub fn get_shadow_culling_plane(self, index: i32) -> crate::unity_engine::plane::Plane;

    #[method(name = "GetCameraCullingPlane", args = 1)]
    pub fn get_camera_culling_plane(self, index: i32) -> crate::unity_engine::plane::Plane;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::cameraproperties::CameraProperties,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
