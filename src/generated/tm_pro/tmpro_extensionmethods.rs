
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmpro_extensionmethods/TMPro_ExtensionMethods.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMPro_ExtensionMethods")]
#[parent(crate::system::object::Object)]
pub struct TMPro_ExtensionMethods {}

#[cfg(feature = "tm_pro-tmpro_extensionmethods")]
#[::unity2::methods]
impl TMPro_ExtensionMethods {
    #[method(name = "ToIntArray", args = 1)]
    pub fn to_int_array(text: ::unity2::Il2CppString) -> ::unity2::Array<i32>;

    #[method(name = "ArrayToString", args = 1)]
    pub fn array_to_string(chars: ::unity2::Array<u16>) -> ::unity2::Il2CppString;

    #[method(name = "IntToString", args = 1)]
    pub fn int_to_string(unicodes: ::unity2::Array<i32>) -> ::unity2::Il2CppString;

    #[method(name = "UintToString", args = 1)]
    pub fn uint_to_string(
        unicodes: crate::system::collections::generic::list_1::List_1<u32>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IntToString", args = 3)]
    pub fn int_to_string_2(
        unicodes: ::unity2::Array<i32>,
        start: i32,
        length: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Compare", args = 2)]
    pub fn compare(
        a: crate::unity_engine::color32::Color32,
        b: crate::unity_engine::color32::Color32,
    ) -> bool;

    #[method(name = "CompareRGB", args = 2)]
    pub fn compare_rgb(
        a: crate::unity_engine::color32::Color32,
        b: crate::unity_engine::color32::Color32,
    ) -> bool;

    #[method(name = "Compare", args = 2)]
    pub fn compare_2(
        a: crate::unity_engine::color::Color,
        b: crate::unity_engine::color::Color,
    ) -> bool;

    #[method(name = "CompareRGB", args = 2)]
    pub fn compare_rgb_2(
        a: crate::unity_engine::color::Color,
        b: crate::unity_engine::color::Color,
    ) -> bool;

    #[method(name = "Multiply", args = 2)]
    pub fn multiply(
        c1: crate::unity_engine::color32::Color32,
        c2: crate::unity_engine::color32::Color32,
    ) -> crate::unity_engine::color32::Color32;

    #[method(name = "Tint", args = 2)]
    pub fn tint(
        c1: crate::unity_engine::color32::Color32,
        c2: crate::unity_engine::color32::Color32,
    ) -> crate::unity_engine::color32::Color32;

    #[method(name = "Tint", args = 2)]
    pub fn tint_2(
        c1: crate::unity_engine::color32::Color32,
        tint: f32,
    ) -> crate::unity_engine::color32::Color32;

    #[method(name = "MinAlpha", args = 2)]
    pub fn min_alpha(
        c1: crate::unity_engine::color::Color,
        c2: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "Compare", args = 3)]
    pub fn compare_3(
        v1: crate::unity_engine::vector3::Vector3,
        v2: crate::unity_engine::vector3::Vector3,
        accuracy: i32,
    ) -> bool;

    #[method(name = "Compare", args = 3)]
    pub fn compare_4(
        q1: crate::unity_engine::quaternion::Quaternion,
        q2: crate::unity_engine::quaternion::Quaternion,
        accuracy: i32,
    ) -> bool;
}
