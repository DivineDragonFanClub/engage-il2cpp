
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/bit/Bit.md")))]
#[::unity2::class(namespace = "Combat", name = "Bit")]
#[parent(crate::system::object::Object)]
pub struct Bit {}

#[cfg(feature = "combat-bit")]
#[::unity2::methods]
impl Bit {
    #[method(name = "Get", args = 3)]
    pub fn get(src_value: i32, bits: i32, shift: i32) -> i32;

    #[method(name = "GetSigned", args = 3)]
    pub fn get_signed(src_value: i32, bits: i32, shift: i32) -> i32;

    #[method(name = "Combine", args = 4)]
    pub fn combine(src_value: i32, value: i32, bits: i32, shift: i32) -> i32;

    #[method(name = "Combine", args = 3)]
    pub fn combine_2(src_value: i32, value: bool, shift: i32) -> i32;
}
