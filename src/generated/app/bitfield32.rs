
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bitfield32/BitField32.md")))]
#[::unity2::class(namespace = "App", name = "BitField32")]
#[parent(crate::app::bitfieldcommon::BitFieldCommon)]
pub struct BitField32 {
    #[rename(name = "m_Value")]
    pub m_value: i32,
}

#[cfg(feature = "app-bitfield32")]
#[::unity2::methods]
impl BitField32 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set(self, f: i32) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, f: i32) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear_2(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "Change", args = 1)]
    pub fn change(self, f: i32) -> ();

    #[method(name = "Change", args = 1)]
    pub fn change_2(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, f: i32) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy_2(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "SetOrClear", args = 2)]
    pub fn set_or_clear(self, is_set: bool, f: i32) -> ();

    #[method(name = "Exclusive", args = 2)]
    pub fn exclusive(self, n: i32, m: i32) -> bool;

    #[method(name = "Mask", args = 1)]
    pub fn mask(self, f: i32) -> i32;

    #[method(name = "Mask", args = 1)]
    pub fn mask_2(self, f: crate::app::bitfield32::BitField32) -> i32;

    #[method(name = "Test", args = 1)]
    pub fn test(self, f: i32) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_2(self, f: crate::app::bitfield32::BitField32) -> bool;

    #[method(name = "Not", args = 1)]
    pub fn not(self, f: i32) -> bool;

    #[method(name = "Not", args = 1)]
    pub fn not_2(self, f: crate::app::bitfield32::BitField32) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset_2(self, f: i32) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset_3(self, f: crate::app::bitfield32::BitField32) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ValueType", args = 0)]
    pub fn get_value_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(lhs: crate::app::bitfield32::BitField32, rhs: i32) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(lhs: crate::app::bitfield32::BitField32, rhs: i32) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality_2(
        lhs: crate::app::bitfield32::BitField32,
        rhs: crate::app::bitfield32::BitField32,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality_2(
        lhs: crate::app::bitfield32::BitField32,
        rhs: crate::app::bitfield32::BitField32,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, rhs_obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, rhs: crate::app::bitfield32::BitField32) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Read", args = 1)]
    pub fn read(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-bitfield32")]
impl BitField32 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BitField32),
                ::core::stringify!(new),
            )
        });
        <Self as IBitField32Methods>::ctor(this);
        this
    }

    pub fn new_2(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BitField32),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBitField32Methods>::ctor_2(this, f);
        this
    }

    pub fn new_3(f: crate::app::bitfield32::BitField32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BitField32),
                ::core::stringify!(new_3),
            )
        });
        <Self as IBitField32Methods>::ctor_3(this, f);
        this
    }
}
