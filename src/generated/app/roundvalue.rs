
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/roundvalue/RoundValue.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RoundValue {
    pub m_min: i32,
    pub m_max: i32,
    pub m_value: i32,
}

impl ::unity2::ClassIdentity for RoundValue {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RoundValue";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RoundValue {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-roundvalue")]
#[::unity2::methods(value)]
impl RoundValue {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, max: i32, value: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(self, min: i32, max: i32, value: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Inc", args = 0)]
    pub fn inc(self) -> ();

    #[method(name = "Dec", args = 0)]
    pub fn dec(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, addend: i32) -> ();

    #[method(name = "Sub", args = 1)]
    pub fn sub(self, subtrahend: i32) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "get_Min", args = 0)]
    pub fn get_min(self) -> i32;

    #[method(name = "get_Max", args = 0)]
    pub fn get_max(self) -> i32;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(value: crate::app::roundvalue::RoundValue) -> i32;
}
