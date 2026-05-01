
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/bitarray8/BitArray8.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BitArray8 {
    pub data: u8,
}

impl ::unity2::ClassIdentity for BitArray8 {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "BitArray8";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BitArray8 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-bitarray8")]
#[::unity2::methods(value)]
impl BitArray8 {
    #[method(name = "get_capacity", args = 0)]
    pub fn get_capacity(self) -> u32;

    #[method(name = "get_allFalse", args = 0)]
    pub fn get_all_false(self) -> bool;

    #[method(name = "get_allTrue", args = 0)]
    pub fn get_all_true(self) -> bool;

    #[method(name = "get_humanizedData", args = 0)]
    pub fn get_humanized_data(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: u32) -> bool;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: u32, value: bool) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, init_value: u8) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        bit_index_true: crate::system::collections::generic::ienumerable_1::IEnumerable_1<u32>,
    ) -> ();

    #[method(name = "op_OnesComplement", args = 1)]
    pub fn op_ones_complement(
        a: crate::unity_engine::rendering::bitarray8::BitArray8,
    ) -> crate::unity_engine::rendering::bitarray8::BitArray8;

    #[method(name = "op_BitwiseOr", args = 2)]
    pub fn op_bitwise_or(
        a: crate::unity_engine::rendering::bitarray8::BitArray8,
        b: crate::unity_engine::rendering::bitarray8::BitArray8,
    ) -> crate::unity_engine::rendering::bitarray8::BitArray8;

    #[method(name = "op_BitwiseAnd", args = 2)]
    pub fn op_bitwise_and(
        a: crate::unity_engine::rendering::bitarray8::BitArray8,
        b: crate::unity_engine::rendering::bitarray8::BitArray8,
    ) -> crate::unity_engine::rendering::bitarray8::BitArray8;

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

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        a: crate::unity_engine::rendering::bitarray8::BitArray8,
        b: crate::unity_engine::rendering::bitarray8::BitArray8,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        a: crate::unity_engine::rendering::bitarray8::BitArray8,
        b: crate::unity_engine::rendering::bitarray8::BitArray8,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
