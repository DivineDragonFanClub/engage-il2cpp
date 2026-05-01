
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/killedbonusdata/KilledBonusData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "KilledBonusData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: killedbonusdata :: KilledBonusData_Flags >)]
pub struct KilledBonusData_FlagField {}

#[cfg(feature = "app-killedbonusdata")]
#[::unity2::methods]
impl KilledBonusData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::killedbonusdata::KilledBonusData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::killedbonusdata::KilledBonusData_Flags) -> i32;
}

#[cfg(feature = "app-killedbonusdata")]
impl KilledBonusData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KilledBonusData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IKilledBonusData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::killedbonusdata::KilledBonusData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KilledBonusData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IKilledBonusData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/killedbonusdata/KilledBonusData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct KilledBonusData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for KilledBonusData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "KilledBonusData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for KilledBonusData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl KilledBonusData_Flags {
    pub fn god() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/killedbonusdata/KilledBonusData.md")))]
#[::unity2::class(namespace = "App", name = "KilledBonusData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: killedbonusdata :: KilledBonusData >)]
pub struct KilledBonusData {}

#[cfg(feature = "app-killedbonusdata")]
#[::unity2::methods]
impl KilledBonusData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::killedbonusdata::KilledBonusData_Kinds;

    #[method(name = "set_Kind", args = 1)]
    pub fn set_kind(self, value: crate::app::killedbonusdata::KilledBonusData_Kinds) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> u8;

    #[method(name = "set_Rate", args = 1)]
    pub fn set_rate(self, value: u8) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::killedbonusdata::KilledBonusData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::killedbonusdata::KilledBonusData_FlagField) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-killedbonusdata")]
impl KilledBonusData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KilledBonusData),
                ::core::stringify!(new),
            )
        });
        <Self as IKilledBonusDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/killedbonusdata/KilledBonusData_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct KilledBonusData_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for KilledBonusData_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "KilledBonusData.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for KilledBonusData_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl KilledBonusData_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn piece_of_bond() -> Self {
        Self { value: 1 }
    }

    pub fn exp() -> Self {
        Self { value: 2 }
    }

    pub fn engage_count() -> Self {
        Self { value: 3 }
    }

    pub fn god_exp() -> Self {
        Self { value: 4 }
    }
}
