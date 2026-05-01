
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/watchtype/WatchType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WatchType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WatchType {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter.Debugging";

    const NAME: &'static str = "WatchType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WatchType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WatchType {
    pub fn watches() -> Self {
        Self { value: 0 }
    }

    pub fn v_stack() -> Self {
        Self { value: 1 }
    }

    pub fn call_stack() -> Self {
        Self { value: 2 }
    }

    pub fn coroutines() -> Self {
        Self { value: 3 }
    }

    pub fn locals() -> Self {
        Self { value: 4 }
    }

    pub fn threads() -> Self {
        Self { value: 5 }
    }

    pub fn max_value() -> Self {
        Self { value: 6 }
    }
}
