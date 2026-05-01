
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/parameterattributes/ParameterAttributes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ParameterAttributes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ParameterAttributes {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "ParameterAttributes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParameterAttributes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ParameterAttributes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn r#in() -> Self {
        Self { value: 1 }
    }

    pub fn out() -> Self {
        Self { value: 2 }
    }

    pub fn lcid() -> Self {
        Self { value: 4 }
    }

    pub fn retval() -> Self {
        Self { value: 8 }
    }

    pub fn optional() -> Self {
        Self { value: 16 }
    }

    pub fn reserved_mask() -> Self {
        Self { value: 61440 }
    }

    pub fn has_default() -> Self {
        Self { value: 4096 }
    }

    pub fn has_field_marshal() -> Self {
        Self { value: 8192 }
    }

    pub fn reserved3() -> Self {
        Self { value: 16384 }
    }

    pub fn reserved4() -> Self {
        Self { value: 32768 }
    }
}
