
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/spritepackingrotation/SpritePackingRotation.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SpritePackingRotation {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SpritePackingRotation {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "SpritePackingRotation";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SpritePackingRotation {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SpritePackingRotation {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn flip_horizontal() -> Self {
        Self { value: 1 }
    }

    pub fn flip_vertical() -> Self {
        Self { value: 2 }
    }

    pub fn rotate180() -> Self {
        Self { value: 3 }
    }

    pub fn any() -> Self {
        Self { value: 15 }
    }
}
