
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/shadowresolution_2/ShadowResolution_2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ShadowResolution_2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ShadowResolution_2 {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "ShadowResolution";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShadowResolution_2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ShadowResolution_2 {
    pub fn _256() -> Self {
        Self { value: 256 }
    }

    pub fn _512() -> Self {
        Self { value: 512 }
    }

    pub fn _1024() -> Self {
        Self { value: 1024 }
    }

    pub fn _2048() -> Self {
        Self { value: 2048 }
    }

    pub fn _4096() -> Self {
        Self { value: 4096 }
    }
}
