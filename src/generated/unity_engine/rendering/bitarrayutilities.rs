
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/bitarrayutilities/BitArrayUtilities.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "BitArrayUtilities")]
#[parent(crate::system::object::Object)]
pub struct BitArrayUtilities {}

#[cfg(feature = "unity_engine-rendering-bitarrayutilities")]
#[::unity2::methods]
impl BitArrayUtilities {
    #[method(name = "Get8", args = 2)]
    pub fn get8(index: u32, data: u8) -> bool;

    #[method(name = "Get16", args = 2)]
    pub fn get16(index: u32, data: u16) -> bool;

    #[method(name = "Get32", args = 2)]
    pub fn get32(index: u32, data: u32) -> bool;

    #[method(name = "Get64", args = 2)]
    pub fn get64(index: u32, data: u64) -> bool;

    #[method(name = "Get128", args = 3)]
    pub fn get128(index: u32, data1: u64, data2: u64) -> bool;

    #[method(name = "Get256", args = 5)]
    pub fn get256(index: u32, data1: u64, data2: u64, data3: u64, data4: u64) -> bool;

    #[method(name = "Set8", args = 3)]
    pub fn set8(index: u32, data: u8, value: bool) -> ();

    #[method(name = "Set16", args = 3)]
    pub fn set16(index: u32, data: u16, value: bool) -> ();

    #[method(name = "Set32", args = 3)]
    pub fn set32(index: u32, data: u32, value: bool) -> ();

    #[method(name = "Set64", args = 3)]
    pub fn set64(index: u32, data: u64, value: bool) -> ();

    #[method(name = "Set128", args = 4)]
    pub fn set128(index: u32, data1: u64, data2: u64, value: bool) -> ();

    #[method(name = "Set256", args = 6)]
    pub fn set256(index: u32, data1: u64, data2: u64, data3: u64, data4: u64, value: bool) -> ();
}
