
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitentrust/UnitEntrust.md")))]
#[::unity2::class(namespace = "App", name = "UnitEntrust")]
#[parent(crate::system::object::Object)]
pub struct UnitEntrust {}

#[cfg(feature = "app-unitentrust")]
#[::unity2::methods]
impl UnitEntrust {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitentrust")]
impl UnitEntrust {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitEntrust),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitEntrustMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitentrust/UnitEntrust_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitEntrust_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitEntrust_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitEntrust.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitEntrust_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitEntrust_Type {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn rush() -> Self {
        Self { value: 1 }
    }

    pub fn escort() -> Self {
        Self { value: 2 }
    }

    pub fn retreat() -> Self {
        Self { value: 3 }
    }

    pub fn advancement() -> Self {
        Self { value: 4 }
    }

    pub fn berserk() -> Self {
        Self { value: 5 }
    }

    pub fn tracking() -> Self {
        Self { value: 6 }
    }

    pub fn num() -> Self {
        Self { value: 7 }
    }
}
