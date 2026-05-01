
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endrolldata/EndRollData_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EndRollData_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EndRollData_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EndRollData.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EndRollData_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EndRollData_Kind {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn staff() -> Self {
        Self { value: 1 }
    }

    pub fn name1() -> Self {
        Self { value: 2 }
    }

    pub fn name2() -> Self {
        Self { value: 3 }
    }

    pub fn name3() -> Self {
        Self { value: 4 }
    }

    pub fn cast() -> Self {
        Self { value: 5 }
    }

    pub fn company_s() -> Self {
        Self { value: 6 }
    }

    pub fn company_m() -> Self {
        Self { value: 7 }
    }

    pub fn company_l() -> Self {
        Self { value: 8 }
    }

    pub fn image() -> Self {
        Self { value: 9 }
    }

    pub fn space_s() -> Self {
        Self { value: 10 }
    }

    pub fn space_m() -> Self {
        Self { value: 11 }
    }

    pub fn space_l() -> Self {
        Self { value: 12 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endrolldata/EndRollData.md")))]
#[::unity2::class(namespace = "App", name = "EndRollData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: endrolldata :: EndRollData >)]
pub struct EndRollData {}

#[cfg(feature = "app-endrolldata")]
#[::unity2::methods]
impl EndRollData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::app::endrolldata::EndRollData_Kind;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: crate::app::endrolldata::EndRollData_Kind) -> ();

    #[method(name = "get_Text1", args = 0)]
    pub fn get_text1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Text1", args = 1)]
    pub fn set_text1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Text2", args = 0)]
    pub fn get_text2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Text2", args = 1)]
    pub fn set_text2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Text3", args = 0)]
    pub fn get_text3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Text3", args = 1)]
    pub fn set_text3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-endrolldata")]
impl EndRollData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EndRollData),
                ::core::stringify!(new),
            )
        });
        <Self as IEndRollDataMethods>::ctor(this);
        this
    }
}
