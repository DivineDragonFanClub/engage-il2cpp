
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_compatibility/TMP_Compatibility.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Compatibility")]
#[parent(crate::system::object::Object)]
pub struct TMP_Compatibility {}

#[cfg(feature = "tm_pro-tmp_compatibility")]
#[::unity2::methods]
impl TMP_Compatibility {
    #[method(name = "ConvertTextAlignmentEnumValues", args = 1)]
    pub fn convert_text_alignment_enum_values(
        old_value: crate::tm_pro::textalignmentoptions::TextAlignmentOptions,
    ) -> crate::tm_pro::textalignmentoptions::TextAlignmentOptions;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_compatibility/TMP_Compatibility_AnchorPositions.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TMP_Compatibility_AnchorPositions {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TMP_Compatibility_AnchorPositions {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_Compatibility.AnchorPositions";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_Compatibility_AnchorPositions {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TMP_Compatibility_AnchorPositions {
    pub fn top_left() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }

    pub fn top_right() -> Self {
        Self { value: 2 }
    }

    pub fn left() -> Self {
        Self { value: 3 }
    }

    pub fn center() -> Self {
        Self { value: 4 }
    }

    pub fn right() -> Self {
        Self { value: 5 }
    }

    pub fn bottom_left() -> Self {
        Self { value: 6 }
    }

    pub fn bottom() -> Self {
        Self { value: 7 }
    }

    pub fn bottom_right() -> Self {
        Self { value: 8 }
    }

    pub fn base_line() -> Self {
        Self { value: 9 }
    }

    pub fn none() -> Self {
        Self { value: 10 }
    }
}
