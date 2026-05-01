
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interopaccessmode/InteropAccessMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct InteropAccessMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for InteropAccessMode {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter";

    const NAME: &'static str = "InteropAccessMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InteropAccessMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl InteropAccessMode {
    pub fn reflection() -> Self {
        Self { value: 0 }
    }

    pub fn lazy_optimized() -> Self {
        Self { value: 1 }
    }

    pub fn preoptimized() -> Self {
        Self { value: 2 }
    }

    pub fn background_optimized() -> Self {
        Self { value: 3 }
    }

    pub fn hardwired() -> Self {
        Self { value: 4 }
    }

    pub fn hide_members() -> Self {
        Self { value: 5 }
    }

    pub fn no_reflection_allowed() -> Self {
        Self { value: 6 }
    }

    pub fn default() -> Self {
        Self { value: 7 }
    }
}
