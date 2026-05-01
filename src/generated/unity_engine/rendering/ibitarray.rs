
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ibitarray/IBitArray.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "IBitArray")]
pub struct IBitArray {}

#[cfg(feature = "unity_engine-rendering-ibitarray")]
#[::unity2::methods]
impl IBitArray {
    #[method(name = "get_capacity", args = 0)]
    pub fn get_capacity(self) -> u32;

    #[method(name = "get_allFalse", args = 0)]
    pub fn get_all_false(self) -> bool;

    #[method(name = "get_allTrue", args = 0)]
    pub fn get_all_true(self) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: u32) -> bool;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: u32, value: bool) -> ();

    #[method(name = "get_humanizedData", args = 0)]
    pub fn get_humanized_data(self) -> ::unity2::Il2CppString;

    #[method(name = "BitAnd", args = 1)]
    pub fn bit_and(
        self,
        other: crate::unity_engine::rendering::ibitarray::IBitArray,
    ) -> crate::unity_engine::rendering::ibitarray::IBitArray;

    #[method(name = "BitOr", args = 1)]
    pub fn bit_or(
        self,
        other: crate::unity_engine::rendering::ibitarray::IBitArray,
    ) -> crate::unity_engine::rendering::ibitarray::IBitArray;

    #[method(name = "BitNot", args = 0)]
    pub fn bit_not(self) -> crate::unity_engine::rendering::ibitarray::IBitArray;
}
