
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/membertypes/MemberTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MemberTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MemberTypes {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "MemberTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MemberTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MemberTypes {
    pub fn constructor() -> Self {
        Self { value: 1 }
    }

    pub fn event() -> Self {
        Self { value: 2 }
    }

    pub fn field() -> Self {
        Self { value: 4 }
    }

    pub fn method() -> Self {
        Self { value: 8 }
    }

    pub fn property() -> Self {
        Self { value: 16 }
    }

    pub fn type_info() -> Self {
        Self { value: 32 }
    }

    pub fn custom() -> Self {
        Self { value: 64 }
    }

    pub fn nested_type() -> Self {
        Self { value: 128 }
    }

    pub fn all() -> Self {
        Self { value: 191 }
    }
}
