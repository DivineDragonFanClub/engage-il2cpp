
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nametypedata/NameTypeData.md")))]
#[::unity2::class(namespace = "App", name = "NameTypeData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: nametypedata :: NameTypeData >)]
pub struct NameTypeData {
    #[static_field]
    #[rename(name = "MaxData")]
    pub max_data: i32,
    #[rename(name = "SuffixArray")]
    pub suffix_array: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-nametypedata")]
#[::unity2::methods]
impl NameTypeData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NameTypes", args = 0)]
    pub fn get_name_types(self) -> ::unity2::Array<crate::app::nametypedata::NameTypeData_Type>;

    #[method(name = "set_NameTypes", args = 1)]
    pub fn set_name_types(
        self,
        value: ::unity2::Array<crate::app::nametypedata::NameTypeData_Type>,
    ) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHubCookSuffix", args = 1)]
    pub fn get_hub_cook_suffix(
        self,
        to_data: crate::app::nametypedata::NameTypeData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsHonorific", args = 1)]
    pub fn is_honorific(r#type: crate::app::nametypedata::NameTypeData_Type) -> bool;

    #[method(name = "get_NameType0", args = 0)]
    pub fn get_name_type0(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType0", args = 1)]
    pub fn set_name_type0(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType1", args = 0)]
    pub fn get_name_type1(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType1", args = 1)]
    pub fn set_name_type1(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType2", args = 0)]
    pub fn get_name_type2(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType2", args = 1)]
    pub fn set_name_type2(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType3", args = 0)]
    pub fn get_name_type3(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType3", args = 1)]
    pub fn set_name_type3(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType4", args = 0)]
    pub fn get_name_type4(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType4", args = 1)]
    pub fn set_name_type4(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType5", args = 0)]
    pub fn get_name_type5(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType5", args = 1)]
    pub fn set_name_type5(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType6", args = 0)]
    pub fn get_name_type6(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType6", args = 1)]
    pub fn set_name_type6(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType7", args = 0)]
    pub fn get_name_type7(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType7", args = 1)]
    pub fn set_name_type7(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType8", args = 0)]
    pub fn get_name_type8(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType8", args = 1)]
    pub fn set_name_type8(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType9", args = 0)]
    pub fn get_name_type9(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType9", args = 1)]
    pub fn set_name_type9(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType10", args = 0)]
    pub fn get_name_type10(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType10", args = 1)]
    pub fn set_name_type10(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType11", args = 0)]
    pub fn get_name_type11(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType11", args = 1)]
    pub fn set_name_type11(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType12", args = 0)]
    pub fn get_name_type12(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType12", args = 1)]
    pub fn set_name_type12(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType13", args = 0)]
    pub fn get_name_type13(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType13", args = 1)]
    pub fn set_name_type13(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType14", args = 0)]
    pub fn get_name_type14(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType14", args = 1)]
    pub fn set_name_type14(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType15", args = 0)]
    pub fn get_name_type15(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType15", args = 1)]
    pub fn set_name_type15(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType16", args = 0)]
    pub fn get_name_type16(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType16", args = 1)]
    pub fn set_name_type16(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType17", args = 0)]
    pub fn get_name_type17(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType17", args = 1)]
    pub fn set_name_type17(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType18", args = 0)]
    pub fn get_name_type18(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType18", args = 1)]
    pub fn set_name_type18(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType19", args = 0)]
    pub fn get_name_type19(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType19", args = 1)]
    pub fn set_name_type19(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType20", args = 0)]
    pub fn get_name_type20(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType20", args = 1)]
    pub fn set_name_type20(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType21", args = 0)]
    pub fn get_name_type21(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType21", args = 1)]
    pub fn set_name_type21(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType22", args = 0)]
    pub fn get_name_type22(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType22", args = 1)]
    pub fn set_name_type22(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType23", args = 0)]
    pub fn get_name_type23(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType23", args = 1)]
    pub fn set_name_type23(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType24", args = 0)]
    pub fn get_name_type24(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType24", args = 1)]
    pub fn set_name_type24(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType25", args = 0)]
    pub fn get_name_type25(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType25", args = 1)]
    pub fn set_name_type25(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType26", args = 0)]
    pub fn get_name_type26(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType26", args = 1)]
    pub fn set_name_type26(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType27", args = 0)]
    pub fn get_name_type27(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType27", args = 1)]
    pub fn set_name_type27(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType28", args = 0)]
    pub fn get_name_type28(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType28", args = 1)]
    pub fn set_name_type28(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType29", args = 0)]
    pub fn get_name_type29(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType29", args = 1)]
    pub fn set_name_type29(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType30", args = 0)]
    pub fn get_name_type30(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType30", args = 1)]
    pub fn set_name_type30(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType31", args = 0)]
    pub fn get_name_type31(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType31", args = 1)]
    pub fn set_name_type31(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType32", args = 0)]
    pub fn get_name_type32(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType32", args = 1)]
    pub fn set_name_type32(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType33", args = 0)]
    pub fn get_name_type33(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType33", args = 1)]
    pub fn set_name_type33(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType34", args = 0)]
    pub fn get_name_type34(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType34", args = 1)]
    pub fn set_name_type34(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType35", args = 0)]
    pub fn get_name_type35(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType35", args = 1)]
    pub fn set_name_type35(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType36", args = 0)]
    pub fn get_name_type36(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType36", args = 1)]
    pub fn set_name_type36(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType37", args = 0)]
    pub fn get_name_type37(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType37", args = 1)]
    pub fn set_name_type37(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType38", args = 0)]
    pub fn get_name_type38(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType38", args = 1)]
    pub fn set_name_type38(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();

    #[method(name = "get_NameType39", args = 0)]
    pub fn get_name_type39(self) -> crate::app::nametypedata::NameTypeData_Type;

    #[method(name = "set_NameType39", args = 1)]
    pub fn set_name_type39(self, value: crate::app::nametypedata::NameTypeData_Type) -> ();
}

#[cfg(feature = "app-nametypedata")]
impl NameTypeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameTypeData),
                ::core::stringify!(new),
            )
        });
        <Self as INameTypeDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/nametypedata/NameTypeData_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NameTypeData_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NameTypeData_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NameTypeData.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NameTypeData_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NameTypeData_Type {
    pub fn _unnamed() -> Self {
        Self { value: 0 }
    }
}
