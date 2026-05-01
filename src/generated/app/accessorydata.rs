
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorydata/AccessoryData_Masks.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AccessoryData_Masks {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AccessoryData_Masks {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AccessoryData.Masks";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AccessoryData_Masks {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AccessoryData_Masks {
    pub fn body() -> Self {
        Self { value: 1 }
    }

    pub fn head() -> Self {
        Self { value: 2 }
    }

    pub fn face() -> Self {
        Self { value: 4 }
    }

    pub fn back() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorydata/AccessoryData.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: accessorydata :: AccessoryData >)]
pub struct AccessoryData {
    #[rename(name = "FlagName")]
    pub flag_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-accessorydata")]
#[::unity2::methods]
impl AccessoryData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Aid", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Aid", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NameM", args = 0)]
    pub fn get_name_m(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NameM", args = 1)]
    pub fn set_name_m(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HelpM", args = 0)]
    pub fn get_help_m(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HelpM", args = 1)]
    pub fn set_help_m(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NameF", args = 0)]
    pub fn get_name_f(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NameF", args = 1)]
    pub fn set_name_f(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HelpF", args = 0)]
    pub fn get_help_f(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HelpF", args = 1)]
    pub fn set_help_f(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_First", args = 0)]
    pub fn get_first(self) -> bool;

    #[method(name = "set_First", args = 1)]
    pub fn set_first(self, value: bool) -> ();

    #[method(name = "get_Amiibo", args = 0)]
    pub fn get_amiibo(self) -> bool;

    #[method(name = "set_Amiibo", args = 1)]
    pub fn set_amiibo(self, value: bool) -> ();

    #[method(name = "get_CondtionCid", args = 0)]
    pub fn get_condtion_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CondtionCid", args = 1)]
    pub fn set_condtion_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CondtionGender", args = 0)]
    pub fn get_condtion_gender(self) -> crate::app::gender::Gender;

    #[method(name = "set_CondtionGender", args = 1)]
    pub fn set_condtion_gender(self, value: crate::app::gender::Gender) -> ();

    #[method(name = "get_CondtionSkills", args = 0)]
    pub fn get_condtion_skills(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_CondtionSkills", args = 1)]
    pub fn set_condtion_skills(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Gid", args = 0)]
    pub fn get_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Gid", args = 1)]
    pub fn set_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Asset", args = 0)]
    pub fn get_asset(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Asset", args = 1)]
    pub fn set_asset(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Price", args = 0)]
    pub fn get_price(self) -> i32;

    #[method(name = "set_Price", args = 1)]
    pub fn set_price(self, value: i32) -> ();

    #[method(name = "get_Iron", args = 0)]
    pub fn get_iron(self) -> i32;

    #[method(name = "set_Iron", args = 1)]
    pub fn set_iron(self, value: i32) -> ();

    #[method(name = "get_Steel", args = 0)]
    pub fn get_steel(self) -> i32;

    #[method(name = "set_Steel", args = 1)]
    pub fn set_steel(self, value: i32) -> ();

    #[method(name = "get_Silver", args = 0)]
    pub fn get_silver(self) -> i32;

    #[method(name = "set_Silver", args = 1)]
    pub fn set_silver(self, value: i32) -> ();

    #[method(name = "get_Mask", args = 0)]
    pub fn get_mask(self) -> crate::app::accessorydata::AccessoryData_Masks;

    #[method(name = "set_Mask", args = 1)]
    pub fn set_mask(self, value: crate::app::accessorydata::AccessoryData_Masks) -> ();

    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "set_Kind", args = 1)]
    pub fn set_kind(self, value: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = "get_GodData", args = 0)]
    pub fn get_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_GodData", args = 1)]
    pub fn set_god_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "GetNum", args = 1)]
    pub fn get_num(data: crate::app::accessorydata::AccessoryData) -> i32;

    #[method(name = "SetNum", args = 2)]
    pub fn set_num(data: crate::app::accessorydata::AccessoryData, num: i32) -> ();

    #[method(name = "CanEquip", args = 1)]
    pub fn can_equip(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsAmiiboOpen", args = 0)]
    pub fn is_amiibo_open(self) -> bool;

    #[method(name = "TryGetFromGodData", args = 1)]
    pub fn try_get_from_god_data(
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::accessorydata::AccessoryData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessorydata")]
impl AccessoryData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryData),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorydata/AccessoryData_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AccessoryData_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AccessoryData_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AccessoryData.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AccessoryData_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AccessoryData_Kinds {
    pub fn body() -> Self {
        Self { value: 0 }
    }

    pub fn head() -> Self {
        Self { value: 1 }
    }

    pub fn face() -> Self {
        Self { value: 2 }
    }

    pub fn back() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}
