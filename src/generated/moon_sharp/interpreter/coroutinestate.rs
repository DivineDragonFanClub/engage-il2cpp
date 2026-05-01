
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/coroutinestate/CoroutineState.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CoroutineState {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CoroutineState {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter";

    const NAME: &'static str = "CoroutineState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CoroutineState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CoroutineState {
    pub fn main() -> Self {
        Self { value: 0 }
    }

    pub fn not_started() -> Self {
        Self { value: 1 }
    }

    pub fn suspended() -> Self {
        Self { value: 2 }
    }

    pub fn force_suspended() -> Self {
        Self { value: 3 }
    }

    pub fn running() -> Self {
        Self { value: 4 }
    }

    pub fn dead() -> Self {
        Self { value: 5 }
    }
}
