
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fxz/FXZ.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FXZ {
    pub x: f32,
    pub z: f32,
}

impl ::unity2::ClassIdentity for FXZ {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "FXZ";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FXZ {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "combat-fxz")]
#[::unity2::methods(value)]
impl FXZ {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, x: f32, z: f32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, rhs: crate::combat::fxz::FXZ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, v: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, v: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> bool;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        a: crate::combat::fxz::FXZ,
        b: crate::combat::fxz::FXZ,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "op_UnaryNegation", args = 1)]
    pub fn op_unary_negation(a: crate::combat::fxz::FXZ) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        a: crate::combat::fxz::FXZ,
        b: crate::combat::fxz::FXZ,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(a: crate::combat::fxz::FXZ, scale: f32) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(scale: f32, a: crate::combat::fxz::FXZ) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Division", args = 2)]
    pub fn op_division(a: crate::combat::fxz::FXZ, scale: f32) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition_2(
        xyz: crate::unity_engine::vector3::Vector3,
        xz: crate::combat::fxz::FXZ,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction_2(
        xyz: crate::unity_engine::vector3::Vector3,
        xz: crate::combat::fxz::FXZ,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(v: crate::combat::fxz::FXZ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(v: crate::unity_engine::vector3::Vector3) -> crate::combat::fxz::FXZ;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_3(v: crate::unity_engine::vector2::Vector2) -> crate::combat::fxz::FXZ;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "get_isZero", args = 0)]
    pub fn get_is_zero(self) -> bool;

    #[method(name = "get_isNotZero", args = 0)]
    pub fn get_is_not_zero(self) -> bool;

    #[method(name = "get_isNaN", args = 0)]
    pub fn get_is_na_n(self) -> bool;

    #[method(name = "get_magnitude", args = 0)]
    pub fn get_magnitude(self) -> f32;

    #[method(name = "get_sqrMagnitude", args = 0)]
    pub fn get_sqr_magnitude(self) -> f32;

    #[method(name = "Distance", args = 2)]
    pub fn distance(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "SqrDistance", args = 2)]
    pub fn sqr_distance(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "AtoB", args = 2)]
    pub fn ato_b(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ)
        -> crate::combat::fxz::FXZ;

    #[method(name = "ToRight", args = 1)]
    pub fn to_right(a: crate::combat::fxz::FXZ) -> crate::combat::fxz::FXZ;

    #[method(name = "get_insideUnitCircle", args = 0)]
    pub fn get_inside_unit_circle() -> crate::combat::fxz::FXZ;

    #[method(name = "Dot", args = 2)]
    pub fn dot(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "Cross", args = 2)]
    pub fn cross(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "Rotate", args = 2)]
    pub fn rotate(a: crate::combat::fxz::FXZ, rad: f32) -> crate::combat::fxz::FXZ;

    #[method(name = "PerpendicularDot", args = 2)]
    pub fn perpendicular_dot(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "RadAB", args = 2)]
    pub fn rad_ab(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "Angle360", args = 2)]
    pub fn angle360(a: crate::combat::fxz::FXZ, b: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "Slerp", args = 3)]
    pub fn slerp(
        a: crate::combat::fxz::FXZ,
        b: crate::combat::fxz::FXZ,
        t: f32,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::combat::fxz::FXZ,
        b: crate::combat::fxz::FXZ,
        t: f32,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "Normalize", args = 0)]
    pub fn normalize(self) -> ();

    #[method(name = "get_normalized", args = 0)]
    pub fn get_normalized(self) -> crate::combat::fxz::FXZ;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
