
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bit_2/Bit_2.md")))]
#[::unity2::class(namespace = "App", name = "Bit")]
#[parent(crate::system::object::Object)]
pub struct Bit_2 {}

#[cfg(feature = "app-bit_2")]
#[::unity2::methods]
impl Bit_2 {
    #[method(name = "Set", args = 2)]
    pub fn set(a: i8, b: i8) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear(a: i8, b: i8) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change(a: i8, b: i8) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(a: u8, b: u8) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_2(a: u8, b: u8) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_2(a: u8, b: u8) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_3(a: i16, b: i16) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_3(a: i16, b: i16) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_3(a: i16, b: i16) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_4(a: u16, b: u16) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_4(a: u16, b: u16) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_4(a: u16, b: u16) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_5(a: i32, b: i32) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_5(a: i32, b: i32) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_5(a: i32, b: i32) -> ();

    #[method(name = "Test", args = 2)]
    pub fn test(a: i32, b: i32) -> bool;

    #[method(name = "Not", args = 2)]
    pub fn not(a: i32, b: i32) -> bool;

    #[method(name = "Match", args = 2)]
    pub fn r#match(a: i32, b: i32) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set_6(a: u32, b: u32) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_6(a: u32, b: u32) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_6(a: u32, b: u32) -> ();

    #[method(name = "Test", args = 2)]
    pub fn test_2(a: u32, b: u32) -> bool;

    #[method(name = "Not", args = 2)]
    pub fn not_2(a: u32, b: u32) -> bool;

    #[method(name = "Match", args = 2)]
    pub fn r#match_2(a: u32, b: u32) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set_7(a: i64, b: i64) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_7(a: i64, b: i64) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_7(a: i64, b: i64) -> ();

    #[method(name = "Test", args = 2)]
    pub fn test_3(a: i64, b: i64) -> bool;

    #[method(name = "Not", args = 2)]
    pub fn not_3(a: i64, b: i64) -> bool;

    #[method(name = "Match", args = 2)]
    pub fn r#match_3(a: i64, b: i64) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set_8(a: u64, b: u64) -> ();

    #[method(name = "Clear", args = 2)]
    pub fn clear_8(a: u64, b: u64) -> ();

    #[method(name = "Change", args = 2)]
    pub fn change_8(a: u64, b: u64) -> ();

    #[method(name = "Test", args = 2)]
    pub fn test_4(a: u64, b: u64) -> bool;

    #[method(name = "Not", args = 2)]
    pub fn not_4(a: u64, b: u64) -> bool;

    #[method(name = "Match", args = 2)]
    pub fn r#match_4(a: u64, b: u64) -> bool;

    #[method(name = "Count", args = 1)]
    pub fn count(value: i32) -> i32;

    #[method(name = "Count", args = 1)]
    pub fn count_2(value: i64) -> i32;

    #[method(name = "Digit", args = 1)]
    pub fn digit(value: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-bit_2")]
impl Bit_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Bit_2),
                ::core::stringify!(new),
            )
        });
        <Self as IBit_2Methods>::ctor(this);
        this
    }
}
