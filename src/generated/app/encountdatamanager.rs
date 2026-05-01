
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountdatamanager/EncountDataManager.md")))]
#[::unity2::class(namespace = "App", name = "EncountDataManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: encountdatamanager :: EncountDataManager >)]
pub struct EncountDataManager {
    #[static_field]
    #[rename(name = "isLoadEncountJobData")]
    pub is_load_encount_job_data: bool,
}

#[cfg(feature = "app-encountdatamanager")]
#[::unity2::methods]
impl EncountDataManager {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetWeapons", args = 2)]
    pub fn get_weapons(
        self,
        rank: i32,
        weapon_mask: crate::app::weaponmask::WeaponMask,
    ) -> ::unity2::Array<crate::app::itemdata::ItemData>;

    #[method(name = "GetNormalWeapon", args = 2)]
    pub fn get_normal_weapon(
        self,
        rank: i32,
        kinds: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = "GetAppendWeaponList", args = 2)]
    pub fn get_append_weapon_list(
        self,
        rank: i32,
        kinds: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = "GetNormalWeaponArrayList", args = 2)]
    pub fn get_normal_weapon_array_list(
        self,
        rank: i32,
        kinds: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<
        crate::app::encountweaponcategorydata::EncountWeaponCategoryData,
    >;

    #[method(name = "GetData", args = 1)]
    pub fn get_data(
        self,
        rank: i32,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<
        crate::app::encountequipdata::EncountEquipData,
    >;

    #[method(name = "ReductDispos", args = 1)]
    pub fn reduct_dispos(
        encount_unit_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::encountunitdata::EncountUnitData,
        >,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::encountunitdata::EncountUnitData,
    >;

    #[method(name = "GetReductRate", args = 0)]
    pub fn get_reduct_rate() -> f32;

    #[method(name = "TryGetRareMoneyItem", args = 2)]
    pub fn try_get_rare_money_item(
        difficulty: crate::app::difficulty::Difficulty,
        nation_level: i32,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-encountdatamanager")]
impl EncountDataManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountDataManager),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountDataManagerMethods>::ctor(this);
        this
    }
}
