
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/crc32/Crc32.md")))]
#[::unity2::class(namespace = "App", name = "Crc32")]
#[parent(crate::system::object::Object)]
pub struct Crc32 {
    #[static_field]
    #[rename(name = "s_Table")]
    pub s_table: ::unity2::Array<u32>,
}

#[cfg(feature = "app-crc32")]
#[::unity2::methods]
impl Crc32 {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Calculate", args = 1)]
    pub fn calculate(buf: ::unity2::Array<u8>) -> u32;

    #[method(name = "Calculate", args = 2)]
    pub fn calculate_2(buf: ::unity2::Array<u8>, len: i32) -> u32;

    #[method(name = "UpdateCrc", args = 3)]
    pub fn update_crc(crc: u32, buf: ::unity2::Array<u8>, len: i32) -> u32;
}
