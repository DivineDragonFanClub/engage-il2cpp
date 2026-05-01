
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/assetname/AssetName_SplitMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AssetName_SplitMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AssetName_SplitMode {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "AssetName.SplitMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssetName_SplitMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AssetName_SplitMode {
    pub fn underscore() -> Self {
        Self { value: 0 }
    }

    pub fn underscore_and_hyphen() -> Self {
        Self { value: 1 }
    }

    pub fn discard_hyphen() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/assetname/AssetName.md")))]
#[::unity2::class(namespace = "Combat", name = "AssetName")]
#[parent(crate::system::object::Object)]
pub struct AssetName {}

#[cfg(feature = "combat-assetname")]
#[::unity2::methods]
impl AssetName {
    #[method(name = "MakeAddressablesPath", args = 1)]
    pub fn make_addressables_path(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Tokenize", args = 4)]
    pub fn tokenize(
        name: ::unity2::Il2CppString,
        split_mode: crate::combat::assetname::AssetName_SplitMode,
        start_index: i32,
        end_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsHighClass", args = 1)]
    pub fn is_high_class(dress_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Is異形", args = 1)]
    pub fn is__(name: ::unity2::Il2CppString) -> bool;
}
