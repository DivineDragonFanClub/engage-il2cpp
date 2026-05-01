
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/bitarray_2/BitArray_2.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BitArray_2 {}

impl ::unity2::ClassIdentity for BitArray_2 {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";

    const NAME: &'static str = "BitArray";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BitArray_2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-universal-internal-bitarray_2")]
#[::unity2::methods(value)]
impl BitArray_2 {
    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsSet", args = 1)]
    pub fn is_set(self, bit_index: i32) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set(self, bit_index: i32, val: bool) -> ();
}
