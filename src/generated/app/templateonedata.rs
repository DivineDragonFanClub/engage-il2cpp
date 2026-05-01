
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/templateonedata/TemplateOneData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TemplateOneData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TemplateOneData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TemplateOneData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TemplateOneData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TemplateOneData_Flags {
    pub fn flag1() -> Self {
        Self { value: 1 }
    }

    pub fn flag2() -> Self {
        Self { value: 2 }
    }

    pub fn flag3() -> Self {
        Self { value: 4 }
    }

    pub fn flag4() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/templateonedata/TemplateOneData.md")))]
#[::unity2::class(namespace = "App", name = "TemplateOneData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: templateonedata :: TemplateOneData >)]
pub struct TemplateOneData {}

#[cfg(feature = "app-templateonedata")]
#[::unity2::methods]
impl TemplateOneData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value1", args = 0)]
    pub fn get_value1(self) -> i32;

    #[method(name = "set_Value1", args = 1)]
    pub fn set_value1(self, value: i32) -> ();

    #[method(name = "get_Value2", args = 0)]
    pub fn get_value2(self) -> f32;

    #[method(name = "set_Value2", args = 1)]
    pub fn set_value2(self, value: f32) -> ();

    #[method(name = "get_Value3", args = 0)]
    pub fn get_value3(self) -> i32;

    #[method(name = "set_Value3", args = 1)]
    pub fn set_value3(self, value: i32) -> ();

    #[method(name = "get_Value4", args = 0)]
    pub fn get_value4(self) -> ::unity2::Array<i32>;

    #[method(name = "set_Value4", args = 1)]
    pub fn set_value4(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "get_Value5", args = 0)]
    pub fn get_value5(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Value5", args = 1)]
    pub fn set_value5(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::templateonedata::TemplateOneData_Flags;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::templateonedata::TemplateOneData_Flags) -> ();

    #[method(name = "get_Sample", args = 0)]
    pub fn get_sample(self) -> crate::app::templateonedata::TemplateOneData_SampleClass;

    #[method(name = "set_Sample", args = 1)]
    pub fn set_sample(self, value: crate::app::templateonedata::TemplateOneData_SampleClass) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-templateonedata")]
impl TemplateOneData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TemplateOneData),
                ::core::stringify!(new),
            )
        });
        <Self as ITemplateOneDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/templateonedata/TemplateOneData_SampleClass.md")))]
#[::unity2::class(namespace = "App", name = "TemplateOneData.SampleClass")]
#[parent(crate::system::object::Object)]
pub struct TemplateOneData_SampleClass {}

#[cfg(feature = "app-templateonedata")]
#[::unity2::methods]
impl TemplateOneData_SampleClass {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-templateonedata")]
impl TemplateOneData_SampleClass {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TemplateOneData_SampleClass),
                ::core::stringify!(new),
            )
        });
        <Self as ITemplateOneData_SampleClassMethods>::ctor(this);
        this
    }
}
