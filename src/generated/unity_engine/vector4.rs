
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vector4/Vector4.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ::unity2::ClassIdentity for Vector4 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Vector4";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Vector4 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-vector4")]
#[::unity2::methods(value)]
impl Vector4 {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> f32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: f32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x: f32, y: f32, z: f32, w: f32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, x: f32, y: f32) -> ();

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::unity_engine::vector4::Vector4,
        b: crate::unity_engine::vector4::Vector4,
        t: f32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::vector4::Vector4) -> bool;

    #[method(name = "Normalize", args = 1)]
    pub fn normalize(
        a: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "get_normalized", args = 0)]
    pub fn get_normalized(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "Dot", args = 2)]
    pub fn dot(
        a: crate::unity_engine::vector4::Vector4,
        b: crate::unity_engine::vector4::Vector4,
    ) -> f32;

    #[method(name = "Magnitude", args = 1)]
    pub fn magnitude(a: crate::unity_engine::vector4::Vector4) -> f32;

    #[method(name = "get_sqrMagnitude", args = 0)]
    pub fn get_sqr_magnitude(self) -> f32;

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::vector4::Vector4;

    #[method(name = "get_one", args = 0)]
    pub fn get_one() -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        a: crate::unity_engine::vector4::Vector4,
        b: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_UnaryNegation", args = 1)]
    pub fn op_unary_negation(
        a: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        a: crate::unity_engine::vector4::Vector4,
        d: f32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        d: f32,
        a: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Division", args = 2)]
    pub fn op_division(
        a: crate::unity_engine::vector4::Vector4,
        d: f32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::vector4::Vector4,
        rhs: crate::unity_engine::vector4::Vector4,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::vector4::Vector4,
        rhs: crate::unity_engine::vector4::Vector4,
    ) -> bool;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        v: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(
        v: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_3(
        v: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
