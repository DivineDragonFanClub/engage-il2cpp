
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rectoffset/RectOffset.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "RectOffset")]
#[parent(crate::system::object::Object)]
pub struct RectOffset {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_SourceStyle")]
    pub m_source_style: ::unity2::IlInstance,
}

#[cfg(feature = "unity_engine-rectoffset")]
#[::unity2::methods]
impl RectOffset {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        source_style: crate::system::object::Object,
        source: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "InternalCreate", args = 0)]
    pub fn internal_create() -> ::unity2::IntPtr;

    #[method(name = "InternalDestroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "get_left", args = 0)]
    pub fn get_left(self) -> i32;

    #[method(name = "get_right", args = 0)]
    pub fn get_right(self) -> i32;

    #[method(name = "get_top", args = 0)]
    pub fn get_top(self) -> i32;

    #[method(name = "get_bottom", args = 0)]
    pub fn get_bottom(self) -> i32;

    #[method(name = "get_horizontal", args = 0)]
    pub fn get_horizontal(self) -> i32;

    #[method(name = "get_vertical", args = 0)]
    pub fn get_vertical(self) -> i32;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, rect: crate::unity_engine::rect::Rect) -> crate::unity_engine::rect::Rect;

    #[method(name = "Remove_Injected", args = 2)]
    pub fn remove_injected(
        self,
        rect: crate::unity_engine::rect::Rect,
        ret: crate::unity_engine::rect::Rect,
    ) -> ();
}

#[cfg(feature = "unity_engine-rectoffset")]
impl RectOffset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RectOffset),
                ::core::stringify!(new),
            )
        });
        <Self as IRectOffsetMethods>::ctor(this);
        this
    }

    pub fn new_2(source_style: crate::system::object::Object, source: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RectOffset),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRectOffsetMethods>::ctor_2(this, source_style, source);
        this
    }
}
