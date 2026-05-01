
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/uintptr/UIntPtr.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct UIntPtr {}

impl ::unity2::ClassIdentity for UIntPtr {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "UIntPtr";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UIntPtr {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-uintptr")]
#[::unity2::methods(value)]
impl UIntPtr {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: u32) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(value1: usize, value2: usize) -> bool;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size() -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
