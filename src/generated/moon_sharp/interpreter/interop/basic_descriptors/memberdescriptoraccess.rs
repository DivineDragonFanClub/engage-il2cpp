
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/basic_descriptors/memberdescriptoraccess/MemberDescriptorAccess.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MemberDescriptorAccess {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MemberDescriptorAccess {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter.Interop.BasicDescriptors";

    const NAME: &'static str = "MemberDescriptorAccess";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MemberDescriptorAccess {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MemberDescriptorAccess {
    pub fn can_read() -> Self {
        Self { value: 1 }
    }

    pub fn can_write() -> Self {
        Self { value: 2 }
    }

    pub fn can_execute() -> Self {
        Self { value: 4 }
    }
}
