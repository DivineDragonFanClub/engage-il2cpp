
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cameraparameter/CameraParameter.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CameraParameter {
    pub position: crate::unity_engine::vector3::Vector3,
    pub euler_angles: crate::unity_engine::vector3::Vector3,
    pub field_of_view: f32,
}

impl ::unity2::ClassIdentity for CameraParameter {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "CameraParameter";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CameraParameter {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-cameraparameter")]
#[::unity2::methods(value)]
impl CameraParameter {
    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "CopyTo", args = 1)]
    pub fn copy_to(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from_2(self, param: crate::app::cameraparameter::CameraParameter) -> ();

    #[method(name = "GetForward", args = 0)]
    pub fn get_forward(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetSide", args = 0)]
    pub fn get_side(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetUp", args = 0)]
    pub fn get_up(self) -> crate::unity_engine::vector3::Vector3;
}
