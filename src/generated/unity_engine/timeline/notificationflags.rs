
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/notificationflags/NotificationFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NotificationFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NotificationFlags {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "NotificationFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NotificationFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NotificationFlags {
    pub fn trigger_in_edit_mode() -> Self {
        Self { value: 131073 }
    }

    pub fn retroactive() -> Self {
        Self { value: 262146 }
    }

    pub fn trigger_once() -> Self {
        Self { value: 131076 }
    }
}
