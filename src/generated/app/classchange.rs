
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchange/ClassChange_ChangeJobData_ProofTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ClassChange_ChangeJobData_ProofTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ClassChange_ChangeJobData_ProofTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ClassChange.ChangeJobData.ProofTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ClassChange_ChangeJobData_ProofTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ClassChange_ChangeJobData_ProofTypes {
    pub fn master() -> Self {
        Self { value: 0 }
    }

    pub fn change() -> Self {
        Self { value: 1 }
    }

    pub fn enchant() -> Self {
        Self { value: 2 }
    }

    pub fn gunner() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchange/ClassChange_ChangeJobData.md")))]
#[::unity2::class(namespace = "App", name = "ClassChange.ChangeJobData")]
#[parent(crate::system::object::Object)]
pub struct ClassChange_ChangeJobData {}

#[cfg(feature = "app-classchange")]
#[::unity2::methods]
impl ClassChange_ChangeJobData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        job: crate::app::jobdata::JobData,
        weapon_mask: crate::app::weaponmask::WeaponMask,
    ) -> ();

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "set_Job", args = 1)]
    pub fn set_job(self, value: crate::app::jobdata::JobData) -> ();

    #[method(name = "get_JobWeaponMask", args = 0)]
    pub fn get_job_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_JobWeaponMask", args = 1)]
    pub fn set_job_weapon_mask(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_OriginalJobWeaponMask", args = 0)]
    pub fn get_original_job_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_OriginalJobWeaponMask", args = 1)]
    pub fn set_original_job_weapon_mask(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_ProofType", args = 0)]
    pub fn get_proof_type(self) -> crate::app::classchange::ClassChange_ChangeJobData_ProofTypes;

    #[method(name = "set_ProofType", args = 1)]
    pub fn set_proof_type(
        self,
        value: crate::app::classchange::ClassChange_ChangeJobData_ProofTypes,
    ) -> ();

    #[method(name = "get_CostLevel", args = 0)]
    pub fn get_cost_level(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CostLevel", args = 1)]
    pub fn set_cost_level(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsEnoughLevel", args = 0)]
    pub fn get_is_enough_level(self) -> bool;

    #[method(name = "set_IsEnoughLevel", args = 1)]
    pub fn set_is_enough_level(self, value: bool) -> ();

    #[method(name = "get_CostWeaponMask", args = 0)]
    pub fn get_cost_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_CostWeaponMask", args = 1)]
    pub fn set_cost_weapon_mask(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_EquippableWeaponMask", args = 0)]
    pub fn get_equippable_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_EquippableWeaponMask", args = 1)]
    pub fn set_equippable_weapon_mask(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_IsEnoughItem", args = 0)]
    pub fn get_is_enough_item(self) -> bool;

    #[method(name = "set_IsEnoughItem", args = 1)]
    pub fn set_is_enough_item(self, value: bool) -> ();

    #[method(name = "get_IsGender", args = 0)]
    pub fn get_is_gender(self) -> bool;

    #[method(name = "set_IsGender", args = 1)]
    pub fn set_is_gender(self, value: bool) -> ();

    #[method(name = "get_IsDefaultJob", args = 0)]
    pub fn get_is_default_job(self) -> bool;

    #[method(name = "set_IsDefaultJob", args = 1)]
    pub fn set_is_default_job(self, value: bool) -> ();

    #[method(name = "get_ExItem1", args = 0)]
    pub fn get_ex_item1(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_ExItem1", args = 1)]
    pub fn set_ex_item1(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_ExItem2", args = 0)]
    pub fn get_ex_item2(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_ExItem2", args = 1)]
    pub fn set_ex_item2(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "GetDispWeaponLevel", args = 3)]
    pub fn get_disp_weapon_level(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
        unit: crate::app::unit::Unit,
        is_up: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetProof", args = 0)]
    pub fn get_proof(self) -> crate::app::itemdata::ItemData;

    #[method(name = "CCCheck", args = 1)]
    pub fn cc_check(self, unit: crate::app::unit::Unit) -> bool;
}

#[cfg(feature = "app-classchange")]
impl ClassChange_ChangeJobData {
    pub fn new(
        job: crate::app::jobdata::JobData,
        weapon_mask: crate::app::weaponmask::WeaponMask,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChange_ChangeJobData),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChange_ChangeJobDataMethods>::ctor(this, job, weapon_mask);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/classchange/ClassChange.md")))]
#[::unity2::class(namespace = "App", name = "ClassChange")]
#[parent(crate::system::object::Object)]
pub struct ClassChange {}

#[cfg(feature = "app-classchange")]
#[::unity2::methods]
impl ClassChange {
    #[method(name = "GetSelectJobList", args = 1)]
    pub fn get_select_job_list(
        job_data: crate::app::jobdata::JobData,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::classchange::ClassChange_ChangeJobData,
    >;

    #[method(name = "GetJobListAll", args = 0)]
    pub fn get_job_list_all() -> crate::system::collections::generic::list_1::List_1<
        crate::app::classchange::ClassChange_ChangeJobData,
    >;

    #[method(name = "GetJobList", args = 2)]
    pub fn get_job_list(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "GetJobListByMaster", args = 1)]
    pub fn get_job_list_by_master(
        unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "GetJobListByChange", args = 1)]
    pub fn get_job_list_by_change(
        unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "AddToListForMaster", args = 3)]
    pub fn add_to_list_for_master(
        job_list: crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>,
        unit: crate::app::unit::Unit,
        high_jobs: crate::system::collections::generic::list_1::List_1<
            crate::app::jobdata::JobData,
        >,
    ) -> ();

    #[method(name = "AddToListForChange", args = 2)]
    pub fn add_to_list_for_change(
        job_list: crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "AddToList", args = 5)]
    pub fn add_to_list(
        job_list: crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>,
        unit: crate::app::unit::Unit,
        job: crate::app::jobdata::JobData,
        include_current_job: bool,
        ignore_aptitude_check: bool,
    ) -> ();

    #[method(name = "GetRelationalJobs", args = 1)]
    pub fn get_relational_jobs(
        job: crate::app::jobdata::JobData,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "IsExists", args = 2)]
    pub fn is_exists(
        job_list: crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>,
        target_job: crate::app::jobdata::JobData,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-classchange")]
impl ClassChange {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChange),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeMethods>::ctor(this);
        this
    }
}
