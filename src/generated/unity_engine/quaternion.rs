
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/quaternion/Quaternion.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ::unity2::ClassIdentity for Quaternion {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Quaternion";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Quaternion {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-quaternion")]
#[::unity2::methods(value)]
impl Quaternion {
    #[method(name = "FromToRotation", args = 2)]
    pub fn from_to_rotation(
        from_direction: crate::unity_engine::vector3::Vector3,
        to_direction: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Inverse", args = 1)]
    pub fn inverse(
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Slerp", args = 3)]
    pub fn slerp(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "SlerpUnclamped", args = 3)]
    pub fn slerp_unclamped(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_FromEulerRad", args = 1)]
    pub fn internal_from_euler_rad(
        euler: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_ToEulerRad", args = 1)]
    pub fn internal_to_euler_rad(
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Internal_ToAxisAngleRad", args = 3)]
    pub fn internal_to_axis_angle_rad(
        q: crate::unity_engine::quaternion::Quaternion,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "AngleAxis", args = 2)]
    pub fn angle_axis(
        angle: f32,
        axis: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "LookRotation", args = 2)]
    pub fn look_rotation(
        forward: crate::unity_engine::vector3::Vector3,
        upwards: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "LookRotation", args = 1)]
    pub fn look_rotation_2(
        forward: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x: f32, y: f32, z: f32, w: f32) -> ();

    #[method(name = "get_identity", args = 0)]
    pub fn get_identity() -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        lhs: crate::unity_engine::quaternion::Quaternion,
        rhs: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        rotation: crate::unity_engine::quaternion::Quaternion,
        point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "IsEqualUsingDot", args = 1)]
    pub fn is_equal_using_dot(dot: f32) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::quaternion::Quaternion,
        rhs: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::quaternion::Quaternion,
        rhs: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "Dot", args = 2)]
    pub fn dot(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
    ) -> f32;

    #[method(name = "Angle", args = 2)]
    pub fn angle(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
    ) -> f32;

    #[method(name = "Internal_MakePositive", args = 1)]
    pub fn internal_make_positive(
        euler: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_eulerAngles", args = 0)]
    pub fn get_euler_angles(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Euler", args = 3)]
    pub fn euler(x: f32, y: f32, z: f32) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Euler", args = 1)]
    pub fn euler_2(
        euler: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "ToAngleAxis", args = 2)]
    pub fn to_angle_axis(self, angle: f32, axis: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "RotateTowards", args = 3)]
    pub fn rotate_towards(
        from: crate::unity_engine::quaternion::Quaternion,
        to: crate::unity_engine::quaternion::Quaternion,
        max_degrees_delta: f32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Normalize", args = 1)]
    pub fn normalize(
        q: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "get_normalized", args = 0)]
    pub fn get_normalized(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::quaternion::Quaternion) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "FromToRotation_Injected", args = 3)]
    pub fn from_to_rotation_injected(
        from_direction: crate::unity_engine::vector3::Vector3,
        to_direction: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Inverse_Injected", args = 2)]
    pub fn inverse_injected(
        rotation: crate::unity_engine::quaternion::Quaternion,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Slerp_Injected", args = 4)]
    pub fn slerp_injected(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SlerpUnclamped_Injected", args = 4)]
    pub fn slerp_unclamped_injected(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Lerp_Injected", args = 4)]
    pub fn lerp_injected(
        a: crate::unity_engine::quaternion::Quaternion,
        b: crate::unity_engine::quaternion::Quaternion,
        t: f32,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_FromEulerRad_Injected", args = 2)]
    pub fn internal_from_euler_rad_injected(
        euler: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_ToEulerRad_Injected", args = 2)]
    pub fn internal_to_euler_rad_injected(
        rotation: crate::unity_engine::quaternion::Quaternion,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Internal_ToAxisAngleRad_Injected", args = 3)]
    pub fn internal_to_axis_angle_rad_injected(
        q: crate::unity_engine::quaternion::Quaternion,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "AngleAxis_Injected", args = 3)]
    pub fn angle_axis_injected(
        angle: f32,
        axis: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "LookRotation_Injected", args = 3)]
    pub fn look_rotation_injected(
        forward: crate::unity_engine::vector3::Vector3,
        upwards: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();
}
