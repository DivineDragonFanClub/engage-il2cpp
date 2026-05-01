
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bitfieldtemplate32_1/BitFieldTemplate32_1.md")))]
#[::unity2::class(namespace = "App", name = "BitFieldTemplate32`1")]
pub struct BitFieldTemplate32_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-bitfieldtemplate32_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> BitFieldTemplate32_1<T0> {
    #[method(name = "get_ValueType", args = 0)]
    pub fn get_value_type(self) -> ::unity2::SystemType;

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: T0) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set(self, f: T0) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, f: T0) -> ();

    #[method(name = "Change", args = 1)]
    pub fn change(self, f: T0) -> ();

    #[method(name = "Mask", args = 1)]
    pub fn mask(self, f: T0) -> i32;

    #[method(name = "Test", args = 1)]
    pub fn test(self, f: T0) -> bool;

    #[method(name = "Not", args = 1)]
    pub fn not(self, f: T0) -> bool;

    #[method(name = "Reset", args = 1)]
    pub fn reset(self, f: T0) -> ();

    #[method(name = "SetOrClear", args = 2)]
    pub fn set_or_clear(self, is_set: bool, f: T0) -> ();

    #[method(name = "Exclusive", args = 2)]
    pub fn exclusive(self, n: T0, m: T0) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-bitfieldtemplate32_1")]
impl<T0: ::unity2::ClassIdentity> BitFieldTemplate32_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BitFieldTemplate32_1),
                ::core::stringify!(new),
            )
        });
        <Self as IBitFieldTemplate32_1Methods<T0>>::ctor(this);
        this
    }
}
