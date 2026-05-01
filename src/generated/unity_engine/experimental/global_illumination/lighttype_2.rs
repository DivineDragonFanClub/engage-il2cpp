
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/global_illumination/lighttype_2/LightType_2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct LightType_2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for LightType_2 {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";

    const NAME: &'static str = "LightType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LightType_2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl LightType_2 {
    pub fn directional() -> Self {
        Self { value: 50462976 }
    }

    pub fn point() -> Self {
        Self { value: 67305985 }
    }

    pub fn spot() -> Self {
        Self { value: 84148994 }
    }

    pub fn rectangle() -> Self {
        Self { value: 100992003 }
    }

    pub fn disc() -> Self {
        Self { value: 394500 }
    }

    pub fn spot_pyramid_shape() -> Self {
        Self { value: 16778757 }
    }

    pub fn spot_box_shape() -> Self {
        Self { value: 33619974 }
    }
}
