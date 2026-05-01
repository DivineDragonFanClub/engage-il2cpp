
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bitstruct/BitStruct.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BitStruct {
    pub m_bits: ::unity2::Array<u8>,
}

impl ::unity2::ClassIdentity for BitStruct {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BitStruct";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BitStruct {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-bitstruct")]
#[::unity2::methods(value)]
impl BitStruct {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, count: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> bool;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: bool) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, index: i32) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set(self, index: i32, enable: bool) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, index: i32) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, index: i32) -> ();

    #[method(name = "SetAll", args = 1)]
    pub fn set_all(self, enable: bool) -> ();

    #[method(name = "Fill", args = 0)]
    pub fn fill(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear_2(self) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_3(self, index: i32, count: i32) -> ();

    #[method(name = "Fill", args = 2)]
    pub fn fill_2(self, index: i32, count: i32) -> ();
}
