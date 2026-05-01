
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chartgoddata/ChartGodData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "ChartGodData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: chartgoddata :: ChartGodData_Flags >)]
pub struct ChartGodData_FlagField {}

#[cfg(feature = "app-chartgoddata")]
#[::unity2::methods]
impl ChartGodData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::chartgoddata::ChartGodData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::chartgoddata::ChartGodData_Flags) -> i32;
}

#[cfg(feature = "app-chartgoddata")]
impl ChartGodData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChartGodData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IChartGodData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::chartgoddata::ChartGodData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChartGodData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IChartGodData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chartgoddata/ChartGodData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ChartGodData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ChartGodData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ChartGodData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ChartGodData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ChartGodData_Flags {
    pub fn add_god_lueur() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chartgoddata/ChartGodData.md")))]
#[::unity2::class(namespace = "App", name = "ChartGodData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: chartgoddata :: ChartGodData >)]
pub struct ChartGodData {}

#[cfg(feature = "app-chartgoddata")]
#[::unity2::methods]
impl ChartGodData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MarthLevel", args = 0)]
    pub fn get_marth_level(self) -> i32;

    #[method(name = "set_MarthLevel", args = 1)]
    pub fn set_marth_level(self, value: i32) -> ();

    #[method(name = "get_SigludLevel", args = 0)]
    pub fn get_siglud_level(self) -> i32;

    #[method(name = "set_SigludLevel", args = 1)]
    pub fn set_siglud_level(self, value: i32) -> ();

    #[method(name = "get_CelicaLevel", args = 0)]
    pub fn get_celica_level(self) -> i32;

    #[method(name = "set_CelicaLevel", args = 1)]
    pub fn set_celica_level(self, value: i32) -> ();

    #[method(name = "get_MicaiahLevel", args = 0)]
    pub fn get_micaiah_level(self) -> i32;

    #[method(name = "set_MicaiahLevel", args = 1)]
    pub fn set_micaiah_level(self, value: i32) -> ();

    #[method(name = "get_RoyLevel", args = 0)]
    pub fn get_roy_level(self) -> i32;

    #[method(name = "set_RoyLevel", args = 1)]
    pub fn set_roy_level(self, value: i32) -> ();

    #[method(name = "get_LeafLevel", args = 0)]
    pub fn get_leaf_level(self) -> i32;

    #[method(name = "set_LeafLevel", args = 1)]
    pub fn set_leaf_level(self, value: i32) -> ();

    #[method(name = "get_LucinaLevel", args = 0)]
    pub fn get_lucina_level(self) -> i32;

    #[method(name = "set_LucinaLevel", args = 1)]
    pub fn set_lucina_level(self, value: i32) -> ();

    #[method(name = "get_LinLevel", args = 0)]
    pub fn get_lin_level(self) -> i32;

    #[method(name = "set_LinLevel", args = 1)]
    pub fn set_lin_level(self, value: i32) -> ();

    #[method(name = "get_IkeLevel", args = 0)]
    pub fn get_ike_level(self) -> i32;

    #[method(name = "set_IkeLevel", args = 1)]
    pub fn set_ike_level(self, value: i32) -> ();

    #[method(name = "get_BylethLevel", args = 0)]
    pub fn get_byleth_level(self) -> i32;

    #[method(name = "set_BylethLevel", args = 1)]
    pub fn set_byleth_level(self, value: i32) -> ();

    #[method(name = "get_KamuiLevel", args = 0)]
    pub fn get_kamui_level(self) -> i32;

    #[method(name = "set_KamuiLevel", args = 1)]
    pub fn set_kamui_level(self, value: i32) -> ();

    #[method(name = "get_EirikLevel", args = 0)]
    pub fn get_eirik_level(self) -> i32;

    #[method(name = "set_EirikLevel", args = 1)]
    pub fn set_eirik_level(self, value: i32) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::chartgoddata::ChartGodData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::chartgoddata::ChartGodData_FlagField) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetLevel", args = 2)]
    pub fn get_level(prefixless_cid: ::unity2::Il2CppString, gid: ::unity2::Il2CppString) -> i32;

    #[method(name = "Gid2Level", args = 1)]
    pub fn gid2_level(self, gid: ::unity2::Il2CppString) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-chartgoddata")]
impl ChartGodData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChartGodData),
                ::core::stringify!(new),
            )
        });
        <Self as IChartGodDataMethods>::ctor(this);
        this
    }
}
