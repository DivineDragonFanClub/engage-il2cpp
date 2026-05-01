
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/valuetype/ValueType.md")))]
#[::unity2::class(namespace = "System", name = "ValueType")]
#[parent(crate::system::object::Object)]
pub struct ValueType {}

#[cfg(feature = "system-valuetype")]
#[::unity2::methods]
impl ValueType {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "InternalEquals", args = 3)]
    pub fn internal_equals(
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        fields: ::unity2::Array<crate::system::object::Object>,
    ) -> bool;

    #[method(name = "DefaultEquals", args = 2)]
    pub fn default_equals(
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "InternalGetHashCode", args = 2)]
    pub fn internal_get_hash_code(
        o: crate::system::object::Object,
        fields: ::unity2::Array<crate::system::object::Object>,
    ) -> i32;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "system-valuetype")]
impl ValueType {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ValueType),
                ::core::stringify!(new),
            )
        });
        <Self as IValueTypeMethods>::ctor(this);
        this
    }
}
