
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/textasset/TextAsset.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "TextAsset")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct TextAsset {}

#[cfg(feature = "unity_engine-textasset")]
#[::unity2::methods]
impl TextAsset {
    #[method(name = "get_bytes", args = 0)]
    pub fn get_bytes(self) -> ::unity2::Array<u8>;

    #[method(name = "GetPreviewBytes", args = 1)]
    pub fn get_preview_bytes(self, max_byte_count: i32) -> ::unity2::Array<u8>;

    #[method(name = "Internal_CreateInstance", args = 2)]
    pub fn internal_create_instance(
        self_: crate::unity_engine::textasset::TextAsset,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        options: crate::unity_engine::textasset::TextAsset_CreateOptions,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetPreview", args = 1)]
    pub fn get_preview(self, max_chars: i32) -> ::unity2::Il2CppString;

    #[method(name = "DecodeString", args = 1)]
    pub fn decode_string(bytes: ::unity2::Array<u8>) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-textasset")]
impl TextAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ITextAssetMethods>::ctor(this);
        this
    }

    pub fn new_2(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextAsset),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITextAssetMethods>::ctor_2(this, text);
        this
    }

    pub fn new_3(
        options: crate::unity_engine::textasset::TextAsset_CreateOptions,
        text: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextAsset),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITextAssetMethods>::ctor_3(this, options, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/textasset/TextAsset_EncodingUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "TextAsset.EncodingUtility")]
#[parent(crate::system::object::Object)]
pub struct TextAsset_EncodingUtility {}

#[cfg(feature = "unity_engine-textasset")]
#[::unity2::methods]
impl TextAsset_EncodingUtility {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/textasset/TextAsset_CreateOptions.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TextAsset_CreateOptions {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TextAsset_CreateOptions {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "TextAsset.CreateOptions";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TextAsset_CreateOptions {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TextAsset_CreateOptions {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn create_native_object() -> Self {
        Self { value: 1 }
    }
}
