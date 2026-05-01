
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bitfieldcommon/BitFieldCommon.md")))]
#[::unity2::class(namespace = "App", name = "BitFieldCommon")]
#[parent(crate::system::object::Object)]
pub struct BitFieldCommon {}

#[cfg(feature = "app-bitfieldcommon")]
#[::unity2::methods]
impl BitFieldCommon {
    #[method(name = "get_ValueType", args = 0)]
    pub fn get_value_type(self) -> ::unity2::SystemType;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-bitfieldcommon")]
impl BitFieldCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BitFieldCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IBitFieldCommonMethods>::ctor(this);
        this
    }
}
