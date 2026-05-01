
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/low_level/glyphpackingmode/GlyphPackingMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GlyphPackingMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GlyphPackingMode {
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";

    const NAME: &'static str = "GlyphPackingMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GlyphPackingMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GlyphPackingMode {
    pub fn best_short_side_fit() -> Self {
        Self { value: 0 }
    }

    pub fn best_long_side_fit() -> Self {
        Self { value: 1 }
    }

    pub fn best_area_fit() -> Self {
        Self { value: 2 }
    }

    pub fn bottom_left_rule() -> Self {
        Self { value: 3 }
    }

    pub fn contact_point_rule() -> Self {
        Self { value: 4 }
    }
}
