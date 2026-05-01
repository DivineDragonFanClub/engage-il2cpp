
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vector2/Vector2.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl ::unity2::ClassIdentity for Vector2 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Vector2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Vector2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-vector2")]
#[::unity2::methods(value)]
impl Vector2 {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> f32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: f32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, x: f32, y: f32) -> ();

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
        t: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "MoveTowards", args = 3)]
    pub fn move_towards(
        current: crate::unity_engine::vector2::Vector2,
        target: crate::unity_engine::vector2::Vector2,
        max_distance_delta: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Scale", args = 2)]
    pub fn scale(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Normalize", args = 0)]
    pub fn normalize(self) -> ();

    #[method(name = "get_normalized", args = 0)]
    pub fn get_normalized(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::vector2::Vector2) -> bool;

    #[method(name = "Dot", args = 2)]
    pub fn dot(
        lhs: crate::unity_engine::vector2::Vector2,
        rhs: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "get_magnitude", args = 0)]
    pub fn get_magnitude(self) -> f32;

    #[method(name = "get_sqrMagnitude", args = 0)]
    pub fn get_sqr_magnitude(self) -> f32;

    #[method(name = "Angle", args = 2)]
    pub fn angle(
        from: crate::unity_engine::vector2::Vector2,
        to: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "SignedAngle", args = 2)]
    pub fn signed_angle(
        from: crate::unity_engine::vector2::Vector2,
        to: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "Distance", args = 2)]
    pub fn distance(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "SqrMagnitude", args = 1)]
    pub fn sqr_magnitude(a: crate::unity_engine::vector2::Vector2) -> f32;

    #[method(name = "Min", args = 2)]
    pub fn min(
        lhs: crate::unity_engine::vector2::Vector2,
        rhs: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Max", args = 2)]
    pub fn max(
        lhs: crate::unity_engine::vector2::Vector2,
        rhs: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Division", args = 2)]
    pub fn op_division(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_UnaryNegation", args = 1)]
    pub fn op_unary_negation(
        a: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        a: crate::unity_engine::vector2::Vector2,
        d: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_3(
        d: f32,
        a: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Division", args = 2)]
    pub fn op_division_2(
        a: crate::unity_engine::vector2::Vector2,
        d: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::vector2::Vector2,
        rhs: crate::unity_engine::vector2::Vector2,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::vector2::Vector2,
        rhs: crate::unity_engine::vector2::Vector2,
    ) -> bool;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        v: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(
        v: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_one", args = 0)]
    pub fn get_one() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_up", args = 0)]
    pub fn get_up() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_down", args = 0)]
    pub fn get_down() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_right", args = 0)]
    pub fn get_right() -> crate::unity_engine::vector2::Vector2;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
