
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/dynamicarray_1/DynamicArray_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DynamicArray`1")]
pub struct DynamicArray_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Array")]
    pub m_array: ::unity2::Array<T0>,
}

#[cfg(feature = "unity_engine-rendering-dynamicarray_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> DynamicArray_1<T0> {
    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: i32) -> ();

    #[method(name = "get_capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, size: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, value: T0) -> i32;

    #[method(name = "Resize", args = 2)]
    pub fn resize(self, new_size: i32, keep_content: bool) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;
}

#[cfg(feature = "unity_engine-rendering-dynamicarray_1")]
impl<T0: ::unity2::ClassIdentity> DynamicArray_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicArray_1),
                ::core::stringify!(new),
            )
        });
        <Self as IDynamicArray_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(size: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicArray_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDynamicArray_1Methods<T0>>::ctor_2(this, size);
        this
    }
}
