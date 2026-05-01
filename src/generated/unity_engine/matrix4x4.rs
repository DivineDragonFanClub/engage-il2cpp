
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/matrix4x4/Matrix4x4.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Matrix4x4 {
    pub m00: f32,
    pub m10: f32,
    pub m20: f32,
    pub m30: f32,
    pub m01: f32,
    pub m11: f32,
    pub m21: f32,
    pub m31: f32,
    pub m02: f32,
    pub m12: f32,
    pub m22: f32,
    pub m32: f32,
    pub m03: f32,
    pub m13: f32,
    pub m23: f32,
    pub m33: f32,
}

impl ::unity2::ClassIdentity for Matrix4x4 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Matrix4x4";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Matrix4x4 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-matrix4x4")]
#[::unity2::methods(value)]
impl Matrix4x4 {
    #[method(name = "GetRotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetLossyScale", args = 0)]
    pub fn get_lossy_scale(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "DecomposeProjection", args = 0)]
    pub fn decompose_projection(self) -> crate::unity_engine::frustumplanes::FrustumPlanes;

    #[method(name = "get_decomposeProjection", args = 0)]
    pub fn get_decompose_projection(self) -> crate::unity_engine::frustumplanes::FrustumPlanes;

    #[method(name = "TRS", args = 3)]
    pub fn trs(
        pos: crate::unity_engine::vector3::Vector3,
        q: crate::unity_engine::quaternion::Quaternion,
        s: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "Inverse", args = 1)]
    pub fn inverse(
        m: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_inverse", args = 0)]
    pub fn get_inverse(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "Perspective", args = 4)]
    pub fn perspective(
        fov: f32,
        aspect: f32,
        z_near: f32,
        z_far: f32,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "LookAt", args = 3)]
    pub fn look_at(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
        up: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        column0: crate::unity_engine::vector4::Vector4,
        column1: crate::unity_engine::vector4::Vector4,
        column2: crate::unity_engine::vector4::Vector4,
        column3: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "set_Item", args = 3)]
    pub fn set_item(self, row: i32, column: i32, value: f32) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> f32;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item_2(self, index: i32, value: f32) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::matrix4x4::Matrix4x4) -> bool;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        lhs: crate::unity_engine::matrix4x4::Matrix4x4,
        rhs: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        lhs: crate::unity_engine::matrix4x4::Matrix4x4,
        vector: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::matrix4x4::Matrix4x4,
        rhs: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::matrix4x4::Matrix4x4,
        rhs: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> bool;

    #[method(name = "GetColumn", args = 1)]
    pub fn get_column(self, index: i32) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetRow", args = 1)]
    pub fn get_row(self, index: i32) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "SetColumn", args = 2)]
    pub fn set_column(self, index: i32, column: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "MultiplyPoint", args = 1)]
    pub fn multiply_point(
        self,
        point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "MultiplyPoint3x4", args = 1)]
    pub fn multiply_point3x4(
        self,
        point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "MultiplyVector", args = 1)]
    pub fn multiply_vector(
        self,
        vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Scale", args = 1)]
    pub fn scale(
        vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "Translate", args = 1)]
    pub fn translate(
        vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "Rotate", args = 1)]
    pub fn rotate(
        q: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_identity", args = 0)]
    pub fn get_identity() -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetRotation_Injected", args = 2)]
    pub fn get_rotation_injected(
        unity_self: crate::unity_engine::matrix4x4::Matrix4x4,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetLossyScale_Injected", args = 2)]
    pub fn get_lossy_scale_injected(
        unity_self: crate::unity_engine::matrix4x4::Matrix4x4,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DecomposeProjection_Injected", args = 2)]
    pub fn decompose_projection_injected(
        unity_self: crate::unity_engine::matrix4x4::Matrix4x4,
        ret: crate::unity_engine::frustumplanes::FrustumPlanes,
    ) -> ();

    #[method(name = "TRS_Injected", args = 4)]
    pub fn trs_injected(
        pos: crate::unity_engine::vector3::Vector3,
        q: crate::unity_engine::quaternion::Quaternion,
        s: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "Inverse_Injected", args = 2)]
    pub fn inverse_injected(
        m: crate::unity_engine::matrix4x4::Matrix4x4,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "Perspective_Injected", args = 5)]
    pub fn perspective_injected(
        fov: f32,
        aspect: f32,
        z_near: f32,
        z_far: f32,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "LookAt_Injected", args = 4)]
    pub fn look_at_injected(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
        up: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();
}
