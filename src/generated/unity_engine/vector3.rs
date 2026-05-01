
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vector3/Vector3.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ::unity2::ClassIdentity for Vector3 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Vector3";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Vector3 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-vector3")]
#[::unity2::methods(value)]
impl Vector3 {
    #[method(name = "Slerp", args = 3)]
    pub fn slerp(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "OrthoNormalize2", args = 2)]
    pub fn ortho_normalize2(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "OrthoNormalize", args = 2)]
    pub fn ortho_normalize(
        normal: crate::unity_engine::vector3::Vector3,
        tangent: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "RotateTowards", args = 4)]
    pub fn rotate_towards(
        current: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        max_radians_delta: f32,
        max_magnitude_delta: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "LerpUnclamped", args = 3)]
    pub fn lerp_unclamped(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "MoveTowards", args = 3)]
    pub fn move_towards(
        current: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        max_distance_delta: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SmoothDamp", args = 4)]
    pub fn smooth_damp(
        current: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        current_velocity: crate::unity_engine::vector3::Vector3,
        smooth_time: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SmoothDamp", args = 6)]
    pub fn smooth_damp_2(
        current: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        current_velocity: crate::unity_engine::vector3::Vector3,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> f32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: f32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, x: f32, y: f32) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set(self, new_x: f32, new_y: f32, new_z: f32) -> ();

    #[method(name = "Scale", args = 2)]
    pub fn scale(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Cross", args = 2)]
    pub fn cross(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "Reflect", args = 2)]
    pub fn reflect(
        in_direction: crate::unity_engine::vector3::Vector3,
        in_normal: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Normalize", args = 1)]
    pub fn normalize(
        value: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Normalize", args = 0)]
    pub fn normalize_2(self) -> ();

    #[method(name = "get_normalized", args = 0)]
    pub fn get_normalized(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Dot", args = 2)]
    pub fn dot(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "Project", args = 2)]
    pub fn project(
        vector: crate::unity_engine::vector3::Vector3,
        on_normal: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ProjectOnPlane", args = 2)]
    pub fn project_on_plane(
        vector: crate::unity_engine::vector3::Vector3,
        plane_normal: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Angle", args = 2)]
    pub fn angle(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "SignedAngle", args = 3)]
    pub fn signed_angle(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
        axis: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "Distance", args = 2)]
    pub fn distance(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "ClampMagnitude", args = 2)]
    pub fn clamp_magnitude(
        vector: crate::unity_engine::vector3::Vector3,
        max_length: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Magnitude", args = 1)]
    pub fn magnitude(vector: crate::unity_engine::vector3::Vector3) -> f32;

    #[method(name = "get_magnitude", args = 0)]
    pub fn get_magnitude(self) -> f32;

    #[method(name = "SqrMagnitude", args = 1)]
    pub fn sqr_magnitude(vector: crate::unity_engine::vector3::Vector3) -> f32;

    #[method(name = "get_sqrMagnitude", args = 0)]
    pub fn get_sqr_magnitude(self) -> f32;

    #[method(name = "Min", args = 2)]
    pub fn min(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Max", args = 2)]
    pub fn max(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_one", args = 0)]
    pub fn get_one() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_forward", args = 0)]
    pub fn get_forward() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_back", args = 0)]
    pub fn get_back() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_up", args = 0)]
    pub fn get_up() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_down", args = 0)]
    pub fn get_down() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_left", args = 0)]
    pub fn get_left() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_right", args = 0)]
    pub fn get_right() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_UnaryNegation", args = 1)]
    pub fn op_unary_negation(
        a: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        a: crate::unity_engine::vector3::Vector3,
        d: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        d: f32,
        a: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Division", args = 2)]
    pub fn op_division(
        a: crate::unity_engine::vector3::Vector3,
        d: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::vector3::Vector3,
        rhs: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "Slerp_Injected", args = 4)]
    pub fn slerp_injected(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "RotateTowards_Injected", args = 5)]
    pub fn rotate_towards_injected(
        current: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        max_radians_delta: f32,
        max_magnitude_delta: f32,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();
}
