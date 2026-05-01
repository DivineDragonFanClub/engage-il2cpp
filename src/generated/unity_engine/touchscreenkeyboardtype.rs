
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/touchscreenkeyboardtype/TouchScreenKeyboardType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TouchScreenKeyboardType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TouchScreenKeyboardType {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "TouchScreenKeyboardType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TouchScreenKeyboardType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TouchScreenKeyboardType {
    pub fn default() -> Self {
        Self { value: 0 }
    }

    pub fn ascii_capable() -> Self {
        Self { value: 1 }
    }

    pub fn numbers_and_punctuation() -> Self {
        Self { value: 2 }
    }

    pub fn url() -> Self {
        Self { value: 3 }
    }

    pub fn number_pad() -> Self {
        Self { value: 4 }
    }

    pub fn phone_pad() -> Self {
        Self { value: 5 }
    }

    pub fn name_phone_pad() -> Self {
        Self { value: 6 }
    }

    pub fn email_address() -> Self {
        Self { value: 7 }
    }

    pub fn nintendo_network_account() -> Self {
        Self { value: 8 }
    }

    pub fn social() -> Self {
        Self { value: 9 }
    }

    pub fn search() -> Self {
        Self { value: 10 }
    }

    pub fn decimal_pad() -> Self {
        Self { value: 11 }
    }

    pub fn one_time_code() -> Self {
        Self { value: 12 }
    }
}
