
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/reliancedata/RelianceData.md")))]
#[::unity2::class(namespace = "App", name = "RelianceData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: reliancedata :: RelianceData >)]
pub struct RelianceData {
    #[static_field]
    #[rename(name = "NoSupport")]
    pub no_support: u8,
    #[static_field]
    #[rename(name = "MaxExp")]
    pub max_exp: i32,
    #[static_field]
    #[rename(name = "MaxData")]
    pub max_data: i32,
}

#[cfg(feature = "app-reliancedata")]
#[::unity2::methods]
impl RelianceData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ExpTypes", args = 0)]
    pub fn get_exp_types(self) -> ::unity2::Array<u8>;

    #[method(name = "set_ExpTypes", args = 1)]
    pub fn set_exp_types(self, value: ::unity2::Array<u8>) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "TryGetExp", args = 1)]
    pub fn try_get_exp(self, index: i32) -> crate::app::relianceexpdata::RelianceExpData;

    #[method(name = "GetLevelText", args = 1)]
    pub fn get_level_text(
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ::unity2::Il2CppString;

    #[method(name = "get_ExpType0", args = 0)]
    pub fn get_exp_type0(self) -> u8;

    #[method(name = "set_ExpType0", args = 1)]
    pub fn set_exp_type0(self, value: u8) -> ();

    #[method(name = "get_ExpType1", args = 0)]
    pub fn get_exp_type1(self) -> u8;

    #[method(name = "set_ExpType1", args = 1)]
    pub fn set_exp_type1(self, value: u8) -> ();

    #[method(name = "get_ExpType2", args = 0)]
    pub fn get_exp_type2(self) -> u8;

    #[method(name = "set_ExpType2", args = 1)]
    pub fn set_exp_type2(self, value: u8) -> ();

    #[method(name = "get_ExpType3", args = 0)]
    pub fn get_exp_type3(self) -> u8;

    #[method(name = "set_ExpType3", args = 1)]
    pub fn set_exp_type3(self, value: u8) -> ();

    #[method(name = "get_ExpType4", args = 0)]
    pub fn get_exp_type4(self) -> u8;

    #[method(name = "set_ExpType4", args = 1)]
    pub fn set_exp_type4(self, value: u8) -> ();

    #[method(name = "get_ExpType5", args = 0)]
    pub fn get_exp_type5(self) -> u8;

    #[method(name = "set_ExpType5", args = 1)]
    pub fn set_exp_type5(self, value: u8) -> ();

    #[method(name = "get_ExpType6", args = 0)]
    pub fn get_exp_type6(self) -> u8;

    #[method(name = "set_ExpType6", args = 1)]
    pub fn set_exp_type6(self, value: u8) -> ();

    #[method(name = "get_ExpType7", args = 0)]
    pub fn get_exp_type7(self) -> u8;

    #[method(name = "set_ExpType7", args = 1)]
    pub fn set_exp_type7(self, value: u8) -> ();

    #[method(name = "get_ExpType8", args = 0)]
    pub fn get_exp_type8(self) -> u8;

    #[method(name = "set_ExpType8", args = 1)]
    pub fn set_exp_type8(self, value: u8) -> ();

    #[method(name = "get_ExpType9", args = 0)]
    pub fn get_exp_type9(self) -> u8;

    #[method(name = "set_ExpType9", args = 1)]
    pub fn set_exp_type9(self, value: u8) -> ();

    #[method(name = "get_ExpType10", args = 0)]
    pub fn get_exp_type10(self) -> u8;

    #[method(name = "set_ExpType10", args = 1)]
    pub fn set_exp_type10(self, value: u8) -> ();

    #[method(name = "get_ExpType11", args = 0)]
    pub fn get_exp_type11(self) -> u8;

    #[method(name = "set_ExpType11", args = 1)]
    pub fn set_exp_type11(self, value: u8) -> ();

    #[method(name = "get_ExpType12", args = 0)]
    pub fn get_exp_type12(self) -> u8;

    #[method(name = "set_ExpType12", args = 1)]
    pub fn set_exp_type12(self, value: u8) -> ();

    #[method(name = "get_ExpType13", args = 0)]
    pub fn get_exp_type13(self) -> u8;

    #[method(name = "set_ExpType13", args = 1)]
    pub fn set_exp_type13(self, value: u8) -> ();

    #[method(name = "get_ExpType14", args = 0)]
    pub fn get_exp_type14(self) -> u8;

    #[method(name = "set_ExpType14", args = 1)]
    pub fn set_exp_type14(self, value: u8) -> ();

    #[method(name = "get_ExpType15", args = 0)]
    pub fn get_exp_type15(self) -> u8;

    #[method(name = "set_ExpType15", args = 1)]
    pub fn set_exp_type15(self, value: u8) -> ();

    #[method(name = "get_ExpType16", args = 0)]
    pub fn get_exp_type16(self) -> u8;

    #[method(name = "set_ExpType16", args = 1)]
    pub fn set_exp_type16(self, value: u8) -> ();

    #[method(name = "get_ExpType17", args = 0)]
    pub fn get_exp_type17(self) -> u8;

    #[method(name = "set_ExpType17", args = 1)]
    pub fn set_exp_type17(self, value: u8) -> ();

    #[method(name = "get_ExpType18", args = 0)]
    pub fn get_exp_type18(self) -> u8;

    #[method(name = "set_ExpType18", args = 1)]
    pub fn set_exp_type18(self, value: u8) -> ();

    #[method(name = "get_ExpType19", args = 0)]
    pub fn get_exp_type19(self) -> u8;

    #[method(name = "set_ExpType19", args = 1)]
    pub fn set_exp_type19(self, value: u8) -> ();

    #[method(name = "get_ExpType20", args = 0)]
    pub fn get_exp_type20(self) -> u8;

    #[method(name = "set_ExpType20", args = 1)]
    pub fn set_exp_type20(self, value: u8) -> ();

    #[method(name = "get_ExpType21", args = 0)]
    pub fn get_exp_type21(self) -> u8;

    #[method(name = "set_ExpType21", args = 1)]
    pub fn set_exp_type21(self, value: u8) -> ();

    #[method(name = "get_ExpType22", args = 0)]
    pub fn get_exp_type22(self) -> u8;

    #[method(name = "set_ExpType22", args = 1)]
    pub fn set_exp_type22(self, value: u8) -> ();

    #[method(name = "get_ExpType23", args = 0)]
    pub fn get_exp_type23(self) -> u8;

    #[method(name = "set_ExpType23", args = 1)]
    pub fn set_exp_type23(self, value: u8) -> ();

    #[method(name = "get_ExpType24", args = 0)]
    pub fn get_exp_type24(self) -> u8;

    #[method(name = "set_ExpType24", args = 1)]
    pub fn set_exp_type24(self, value: u8) -> ();

    #[method(name = "get_ExpType25", args = 0)]
    pub fn get_exp_type25(self) -> u8;

    #[method(name = "set_ExpType25", args = 1)]
    pub fn set_exp_type25(self, value: u8) -> ();

    #[method(name = "get_ExpType26", args = 0)]
    pub fn get_exp_type26(self) -> u8;

    #[method(name = "set_ExpType26", args = 1)]
    pub fn set_exp_type26(self, value: u8) -> ();

    #[method(name = "get_ExpType27", args = 0)]
    pub fn get_exp_type27(self) -> u8;

    #[method(name = "set_ExpType27", args = 1)]
    pub fn set_exp_type27(self, value: u8) -> ();

    #[method(name = "get_ExpType28", args = 0)]
    pub fn get_exp_type28(self) -> u8;

    #[method(name = "set_ExpType28", args = 1)]
    pub fn set_exp_type28(self, value: u8) -> ();

    #[method(name = "get_ExpType29", args = 0)]
    pub fn get_exp_type29(self) -> u8;

    #[method(name = "set_ExpType29", args = 1)]
    pub fn set_exp_type29(self, value: u8) -> ();

    #[method(name = "get_ExpType30", args = 0)]
    pub fn get_exp_type30(self) -> u8;

    #[method(name = "set_ExpType30", args = 1)]
    pub fn set_exp_type30(self, value: u8) -> ();

    #[method(name = "get_ExpType31", args = 0)]
    pub fn get_exp_type31(self) -> u8;

    #[method(name = "set_ExpType31", args = 1)]
    pub fn set_exp_type31(self, value: u8) -> ();

    #[method(name = "get_ExpType32", args = 0)]
    pub fn get_exp_type32(self) -> u8;

    #[method(name = "set_ExpType32", args = 1)]
    pub fn set_exp_type32(self, value: u8) -> ();

    #[method(name = "get_ExpType33", args = 0)]
    pub fn get_exp_type33(self) -> u8;

    #[method(name = "set_ExpType33", args = 1)]
    pub fn set_exp_type33(self, value: u8) -> ();

    #[method(name = "get_ExpType34", args = 0)]
    pub fn get_exp_type34(self) -> u8;

    #[method(name = "set_ExpType34", args = 1)]
    pub fn set_exp_type34(self, value: u8) -> ();

    #[method(name = "get_ExpType35", args = 0)]
    pub fn get_exp_type35(self) -> u8;

    #[method(name = "set_ExpType35", args = 1)]
    pub fn set_exp_type35(self, value: u8) -> ();

    #[method(name = "get_ExpType36", args = 0)]
    pub fn get_exp_type36(self) -> u8;

    #[method(name = "set_ExpType36", args = 1)]
    pub fn set_exp_type36(self, value: u8) -> ();

    #[method(name = "get_ExpType37", args = 0)]
    pub fn get_exp_type37(self) -> u8;

    #[method(name = "set_ExpType37", args = 1)]
    pub fn set_exp_type37(self, value: u8) -> ();

    #[method(name = "get_ExpType38", args = 0)]
    pub fn get_exp_type38(self) -> u8;

    #[method(name = "set_ExpType38", args = 1)]
    pub fn set_exp_type38(self, value: u8) -> ();

    #[method(name = "get_ExpType39", args = 0)]
    pub fn get_exp_type39(self) -> u8;

    #[method(name = "set_ExpType39", args = 1)]
    pub fn set_exp_type39(self, value: u8) -> ();

    #[method(name = "get_ExpType40", args = 0)]
    pub fn get_exp_type40(self) -> u8;

    #[method(name = "set_ExpType40", args = 1)]
    pub fn set_exp_type40(self, value: u8) -> ();

    #[method(name = "get_ExpType41", args = 0)]
    pub fn get_exp_type41(self) -> u8;

    #[method(name = "set_ExpType41", args = 1)]
    pub fn set_exp_type41(self, value: u8) -> ();
}

#[cfg(feature = "app-reliancedata")]
impl RelianceData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelianceData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelianceDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/reliancedata/RelianceData_Level.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelianceData_Level {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelianceData_Level {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelianceData.Level";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelianceData_Level {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelianceData_Level {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn c() -> Self {
        Self { value: 1 }
    }

    pub fn b() -> Self {
        Self { value: 2 }
    }

    pub fn a() -> Self {
        Self { value: 3 }
    }

    pub fn a_plus() -> Self {
        Self { value: 4 }
    }

    pub fn num() -> Self {
        Self { value: 5 }
    }
}
