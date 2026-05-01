
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/global_illumination/fallofftype/FalloffType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FalloffType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FalloffType {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";

    const NAME: &'static str = "FalloffType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FalloffType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FalloffType {
    pub fn inverse_squared() -> Self {
        Self { value: 50462976 }
    }

    pub fn inverse_squared_no_range_attenuation() -> Self {
        Self { value: 67305985 }
    }

    pub fn linear() -> Self {
        Self { value: 262914 }
    }

    pub fn legacy() -> Self {
        Self { value: 16778243 }
    }

    pub fn undefined() -> Self {
        Self { value: 65540 }
    }
}
