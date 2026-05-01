
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assemblynameflags/AssemblyNameFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AssemblyNameFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AssemblyNameFlags {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "AssemblyNameFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssemblyNameFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AssemblyNameFlags {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn public_key() -> Self {
        Self { value: 1 }
    }

    pub fn enable_ji_tcompile_optimizer() -> Self {
        Self { value: 16384 }
    }

    pub fn enable_ji_tcompile_tracking() -> Self {
        Self { value: 32768 }
    }

    pub fn retargetable() -> Self {
        Self { value: 256 }
    }
}
