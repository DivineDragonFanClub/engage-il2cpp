
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/internal/defaultvalueattribute/DefaultValueAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Internal", name = "DefaultValueAttribute")]
pub struct DefaultValueAttribute {
    #[rename(name = "DefaultValue")]
    pub default_value: ::unity2::IlInstance,
}

#[cfg(feature = "unity_engine-internal-defaultvalueattribute")]
#[::unity2::methods]
impl DefaultValueAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "unity_engine-internal-defaultvalueattribute")]
impl DefaultValueAttribute {
    pub fn new(value: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DefaultValueAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDefaultValueAttributeMethods>::ctor(this, value);
        this
    }
}
