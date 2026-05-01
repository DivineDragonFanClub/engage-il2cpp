
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/color32/Color32.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Color32 {
    pub rgba: i32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ::unity2::ClassIdentity for Color32 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Color32";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Color32 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-color32")]
#[::unity2::methods(value)]
impl Color32 {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, r: u8, g: u8, b: u8, a: u8) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        c: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color32::Color32;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(
        c: crate::unity_engine::color32::Color32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
