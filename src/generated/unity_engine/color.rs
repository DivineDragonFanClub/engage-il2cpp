
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/color/Color.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ::unity2::ClassIdentity for Color {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Color";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Color {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-color")]
#[::unity2::methods(value)]
impl Color {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, r: f32, g: f32, b: f32, a: f32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(self, r: f32, g: f32, b: f32) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::color::Color) -> bool;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        a: crate::unity_engine::color::Color,
        b: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        a: crate::unity_engine::color::Color,
        b: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_2(
        a: crate::unity_engine::color::Color,
        b: f32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply_3(
        b: f32,
        a: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::color::Color,
        rhs: crate::unity_engine::color::Color,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::color::Color,
        rhs: crate::unity_engine::color::Color,
    ) -> bool;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        a: crate::unity_engine::color::Color,
        b: crate::unity_engine::color::Color,
        t: f32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "RGBMultiplied", args = 1)]
    pub fn rgb_multiplied(self, multiplier: f32) -> crate::unity_engine::color::Color;

    #[method(name = "get_red", args = 0)]
    pub fn get_red() -> crate::unity_engine::color::Color;

    #[method(name = "get_green", args = 0)]
    pub fn get_green() -> crate::unity_engine::color::Color;

    #[method(name = "get_blue", args = 0)]
    pub fn get_blue() -> crate::unity_engine::color::Color;

    #[method(name = "get_white", args = 0)]
    pub fn get_white() -> crate::unity_engine::color::Color;

    #[method(name = "get_black", args = 0)]
    pub fn get_black() -> crate::unity_engine::color::Color;

    #[method(name = "get_yellow", args = 0)]
    pub fn get_yellow() -> crate::unity_engine::color::Color;

    #[method(name = "get_cyan", args = 0)]
    pub fn get_cyan() -> crate::unity_engine::color::Color;

    #[method(name = "get_magenta", args = 0)]
    pub fn get_magenta() -> crate::unity_engine::color::Color;

    #[method(name = "get_gray", args = 0)]
    pub fn get_gray() -> crate::unity_engine::color::Color;

    #[method(name = "get_grey", args = 0)]
    pub fn get_grey() -> crate::unity_engine::color::Color;

    #[method(name = "get_clear", args = 0)]
    pub fn get_clear() -> crate::unity_engine::color::Color;

    #[method(name = "get_linear", args = 0)]
    pub fn get_linear(self) -> crate::unity_engine::color::Color;

    #[method(name = "get_gamma", args = 0)]
    pub fn get_gamma(self) -> crate::unity_engine::color::Color;

    #[method(name = "get_maxColorComponent", args = 0)]
    pub fn get_max_color_component(self) -> f32;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        c: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(
        v: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::color::Color;
}
