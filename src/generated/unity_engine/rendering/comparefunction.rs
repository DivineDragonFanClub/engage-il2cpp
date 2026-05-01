
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/comparefunction/CompareFunction.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CompareFunction {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CompareFunction {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "CompareFunction";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CompareFunction {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CompareFunction {
    pub fn disabled() -> Self {
        Self { value: 0 }
    }

    pub fn never() -> Self {
        Self { value: 1 }
    }

    pub fn less() -> Self {
        Self { value: 2 }
    }

    pub fn equal() -> Self {
        Self { value: 3 }
    }

    pub fn less_equal() -> Self {
        Self { value: 4 }
    }

    pub fn greater() -> Self {
        Self { value: 5 }
    }

    pub fn not_equal() -> Self {
        Self { value: 6 }
    }

    pub fn greater_equal() -> Self {
        Self { value: 7 }
    }

    pub fn always() -> Self {
        Self { value: 8 }
    }
}
