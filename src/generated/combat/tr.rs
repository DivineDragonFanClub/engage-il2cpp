
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/tr/TR.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TR {
    pub position: crate::unity_engine::vector3::Vector3,
    pub rotation: crate::unity_engine::quaternion::Quaternion,
}

impl ::unity2::ClassIdentity for TR {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "TR";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TR {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "combat-tr")]
#[::unity2::methods(value)]
impl TR {
    #[method(name = "get_identity", args = 0)]
    pub fn get_identity() -> crate::combat::tr::TR;

    #[method(name = "get_forward", args = 0)]
    pub fn get_forward(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_forward", args = 1)]
    pub fn set_forward(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        rot: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        forward: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, t: crate::unity_engine::transform::Transform) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, rhs: crate::combat::tr::TR) -> ();

    #[method(name = "Lerp", args = 4)]
    pub fn lerp(
        a: crate::combat::tr::TR,
        b: crate::combat::tr::TR,
        t: f32,
        c: crate::combat::tr::TR,
    ) -> ();

    #[method(name = "TransformPoint", args = 1)]
    pub fn transform_point(
        self,
        in_pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "LerpAndMakeMatrix", args = 5)]
    pub fn lerp_and_make_matrix(
        a: crate::combat::tr::TR,
        b: crate::combat::tr::TR,
        scale: f32,
        t: f32,
        mtx: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();
}
