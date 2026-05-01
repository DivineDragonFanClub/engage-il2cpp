
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/assettype/AssetType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AssetType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AssetType {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "AssetType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssetType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AssetType {
    pub fn body_model() -> Self {
        Self { value: 0 }
    }

    pub fn dress_model() -> Self {
        Self { value: 1 }
    }

    pub fn head_model() -> Self {
        Self { value: 2 }
    }

    pub fn hair_model() -> Self {
        Self { value: 3 }
    }

    pub fn ride_model() -> Self {
        Self { value: 4 }
    }

    pub fn ride_dress_model() -> Self {
        Self { value: 5 }
    }

    pub fn left_hand_model() -> Self {
        Self { value: 6 }
    }

    pub fn right_hand_model() -> Self {
        Self { value: 7 }
    }

    pub fn private_effect_catalog() -> Self {
        Self { value: 8 }
    }

    pub fn magic_prefab() -> Self {
        Self { value: 9 }
    }

    pub fn body_anim() -> Self {
        Self { value: 10 }
    }

    pub fn ride_anim() -> Self {
        Self { value: 11 }
    }

    pub fn acc0_model() -> Self {
        Self { value: 12 }
    }

    pub fn acc1_model() -> Self {
        Self { value: 13 }
    }

    pub fn acc2_model() -> Self {
        Self { value: 14 }
    }

    pub fn acc3_model() -> Self {
        Self { value: 15 }
    }

    pub fn acc4_model() -> Self {
        Self { value: 16 }
    }

    pub fn acc5_model() -> Self {
        Self { value: 17 }
    }

    pub fn acc6_model() -> Self {
        Self { value: 18 }
    }

    pub fn acc7_model() -> Self {
        Self { value: 19 }
    }

    pub fn max() -> Self {
        Self { value: 20 }
    }
}
