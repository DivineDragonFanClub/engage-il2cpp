
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rectint/RectInt.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RectInt {
    pub m_x_min: i32,
    pub m_y_min: i32,
    pub m_width: i32,
    pub m_height: i32,
}

impl ::unity2::ClassIdentity for RectInt {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "RectInt";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RectInt {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rectint")]
#[::unity2::methods(value)]
impl RectInt {
    #[method(name = "get_x", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_x", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_y", args = 0)]
    pub fn get_y(self) -> i32;

    #[method(name = "set_y", args = 1)]
    pub fn set_y(self, value: i32) -> ();

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> i32;

    #[method(name = "set_width", args = 1)]
    pub fn set_width(self, value: i32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> i32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x_min: i32, y_min: i32, width: i32, height: i32) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::rectint::RectInt) -> bool;
}
