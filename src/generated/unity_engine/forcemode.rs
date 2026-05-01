
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/forcemode/ForceMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ForceMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ForceMode {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ForceMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ForceMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ForceMode {
    pub fn force() -> Self {
        Self { value: 0 }
    }

    pub fn acceleration() -> Self {
        Self { value: 5 }
    }

    pub fn impulse() -> Self {
        Self { value: 1 }
    }

    pub fn velocity_change() -> Self {
        Self { value: 2 }
    }
}
