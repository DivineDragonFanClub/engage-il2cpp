
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/shadowcastingmode/ShadowCastingMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ShadowCastingMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ShadowCastingMode {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ShadowCastingMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShadowCastingMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ShadowCastingMode {
    pub fn off() -> Self {
        Self { value: 0 }
    }

    pub fn on() -> Self {
        Self { value: 1 }
    }

    pub fn two_sided() -> Self {
        Self { value: 2 }
    }

    pub fn shadows_only() -> Self {
        Self { value: 3 }
    }
}
