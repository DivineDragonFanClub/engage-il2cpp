
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rigidbodyconstraints/RigidbodyConstraints.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RigidbodyConstraints {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RigidbodyConstraints {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "RigidbodyConstraints";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RigidbodyConstraints {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RigidbodyConstraints {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn freeze_position_x() -> Self {
        Self { value: 2 }
    }

    pub fn freeze_position_y() -> Self {
        Self { value: 4 }
    }

    pub fn freeze_position_z() -> Self {
        Self { value: 8 }
    }

    pub fn freeze_rotation_x() -> Self {
        Self { value: 16 }
    }

    pub fn freeze_rotation_y() -> Self {
        Self { value: 32 }
    }

    pub fn freeze_rotation_z() -> Self {
        Self { value: 64 }
    }

    pub fn freeze_position() -> Self {
        Self { value: 14 }
    }

    pub fn freeze_rotation() -> Self {
        Self { value: 112 }
    }

    pub fn freeze_all() -> Self {
        Self { value: 126 }
    }
}
