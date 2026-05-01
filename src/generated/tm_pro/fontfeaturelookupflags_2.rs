
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/fontfeaturelookupflags_2/FontFeatureLookupFlags_2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FontFeatureLookupFlags_2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FontFeatureLookupFlags_2 {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "FontFeatureLookupFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FontFeatureLookupFlags_2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FontFeatureLookupFlags_2 {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_ligatures() -> Self {
        Self { value: 4 }
    }

    pub fn ignore_spacing_adjustments() -> Self {
        Self { value: 256 }
    }
}
