
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountunitdata/EncountUnitData_RareType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EncountUnitData_RareType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EncountUnitData_RareType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EncountUnitData.RareType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EncountUnitData_RareType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EncountUnitData_RareType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn exp() -> Self {
        Self { value: 1 }
    }

    pub fn gold() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountunitdata/EncountUnitData.md")))]
#[::unity2::class(namespace = "App", name = "EncountUnitData")]
#[parent(crate::system::object::Object)]
pub struct EncountUnitData {
    #[rename(name = "m_DisposData")]
    pub m_dispos_data: crate::app::disposdata::DisposData,
    #[rename(name = "m_RareType")]
    pub m_rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    #[rename(name = "m_JobData")]
    pub m_job_data: crate::app::jobdata::JobData,
}

#[cfg(feature = "app-encountunitdata")]
#[::unity2::methods]
impl EncountUnitData {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "SetRareType", args = 1)]
    pub fn set_rare_type(self, r#type: crate::app::encountunitdata::EncountUnitData_RareType)
        -> ();

    #[method(name = "get_Dispos", args = 0)]
    pub fn get_dispos(self) -> crate::app::disposdata::DisposData;

    #[method(name = "get_Rare", args = 0)]
    pub fn get_rare(self) -> crate::app::encountunitdata::EncountUnitData_RareType;

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "IsGunner", args = 0)]
    pub fn is_gunner(self) -> bool;

    #[method(name = "IsWolf", args = 0)]
    pub fn is_wolf(self) -> bool;

    #[method(name = "IsDragon", args = 0)]
    pub fn is_dragon(self) -> bool;

    #[method(name = "CreateList", args = 2)]
    pub fn create_list(
        dispos_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
        is_dlc: bool,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::encountunitdata::EncountUnitData,
    >;

    #[method(name = "CreateJobData", args = 1)]
    pub fn create_job_data(
        unit_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::encountunitdata::EncountUnitData,
        >,
    ) -> ();

    #[method(name = "CreatePackOfWolves", args = 1)]
    pub fn create_pack_of_wolves(
        unit_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::encountunitdata::EncountUnitData,
        >,
    ) -> ();

    #[method(name = "SetRareFlags", args = 2)]
    pub fn set_rare_flags(
        encount_unit_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::encountunitdata::EncountUnitData,
        >,
        is_dlc: bool,
    ) -> ();

    #[method(name = "AppendSkill", args = 2)]
    pub fn append_skill(level: i32, data: crate::app::jobdata::JobData) -> ::unity2::Il2CppString;

    #[method(name = "GetRandomKillingSid", args = 0)]
    pub fn get_random_killing_sid() -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-encountunitdata")]
impl EncountUnitData {
    pub fn new(data: crate::app::disposdata::DisposData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountUnitData),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountUnitDataMethods>::ctor(this, data);
        this
    }
}
