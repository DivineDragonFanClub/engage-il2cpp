
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/addressablesplatform/AddressablesPlatform.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AddressablesPlatform {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AddressablesPlatform {
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets";

    const NAME: &'static str = "AddressablesPlatform";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AddressablesPlatform {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AddressablesPlatform {
    pub fn unknown() -> Self {
        Self { value: 0 }
    }

    pub fn windows() -> Self {
        Self { value: 1 }
    }

    pub fn osx() -> Self {
        Self { value: 2 }
    }

    pub fn linux() -> Self {
        Self { value: 3 }
    }

    pub fn ps4() -> Self {
        Self { value: 4 }
    }

    pub fn switch() -> Self {
        Self { value: 5 }
    }

    pub fn xbox_one() -> Self {
        Self { value: 6 }
    }

    pub fn web_gl() -> Self {
        Self { value: 7 }
    }

    pub fn i_os() -> Self {
        Self { value: 8 }
    }

    pub fn android() -> Self {
        Self { value: 9 }
    }

    pub fn windows_universal() -> Self {
        Self { value: 10 }
    }
}
