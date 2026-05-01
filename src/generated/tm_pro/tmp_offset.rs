
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_offset/TMP_Offset.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_Offset {
    pub m_left: f32,
    pub m_right: f32,
    pub m_top: f32,
    pub m_bottom: f32,
}

impl ::unity2::ClassIdentity for TMP_Offset {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_Offset";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_Offset {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-tmp_offset")]
#[::unity2::methods(value)]
impl TMP_Offset {
    #[method(name = "get_left", args = 0)]
    pub fn get_left(self) -> f32;

    #[method(name = "set_left", args = 1)]
    pub fn set_left(self, value: f32) -> ();

    #[method(name = "get_right", args = 0)]
    pub fn get_right(self) -> f32;

    #[method(name = "set_right", args = 1)]
    pub fn set_right(self, value: f32) -> ();

    #[method(name = "get_top", args = 0)]
    pub fn get_top(self) -> f32;

    #[method(name = "set_top", args = 1)]
    pub fn set_top(self, value: f32) -> ();

    #[method(name = "get_bottom", args = 0)]
    pub fn get_bottom(self) -> f32;

    #[method(name = "set_bottom", args = 1)]
    pub fn set_bottom(self, value: f32) -> ();

    #[method(name = "get_horizontal", args = 0)]
    pub fn get_horizontal(self) -> f32;

    #[method(name = "set_horizontal", args = 1)]
    pub fn set_horizontal(self, value: f32) -> ();

    #[method(name = "get_vertical", args = 0)]
    pub fn get_vertical(self) -> f32;

    #[method(name = "set_vertical", args = 1)]
    pub fn set_vertical(self, value: f32) -> ();

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::tm_pro::tmp_offset::TMP_Offset;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, left: f32, right: f32, top: f32, bottom: f32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, horizontal: f32, vertical: f32) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::tm_pro::tmp_offset::TMP_Offset,
        rhs: crate::tm_pro::tmp_offset::TMP_Offset,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::tm_pro::tmp_offset::TMP_Offset,
        rhs: crate::tm_pro::tmp_offset::TMP_Offset,
    ) -> bool;

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        a: crate::tm_pro::tmp_offset::TMP_Offset,
        b: f32,
    ) -> crate::tm_pro::tmp_offset::TMP_Offset;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::tm_pro::tmp_offset::TMP_Offset) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
