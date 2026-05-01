
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/vec3/Vec3.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ::unity2::ClassIdentity for Vec3 {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet";

    const NAME: &'static str = "Vec3";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Vec3 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-vec3")]
#[::unity2::methods(value)]
impl Vec3 {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> f32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: f32) -> ();

    #[method(name = "Sub", args = 3)]
    pub fn sub(
        lhs: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
        rhs: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
        result : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: vec3 :: Vec3,
    ) -> ();

    #[method(name = "Neg", args = 1)]
    pub fn neg(
        v: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
    ) -> ();

    #[method(name = "Dot", args = 3)]
    pub fn dot(
        u: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
        v: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
        dot: f32,
    ) -> ();

    #[method(name = "Normalize", args = 1)]
    pub fn normalize(
        v: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
    ) -> ();

    #[method(name = "LongAxis", args = 1)]
    pub fn long_axis(
        v: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
    ) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
