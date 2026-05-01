
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/report/Report.md")))]
#[::unity2::class(namespace = "App", name = "Report")]
#[parent(crate::system::object::Object)]
pub struct Report {
    #[static_field]
    #[rename(name = "Header")]
    pub header: ::unity2::Il2CppString,
}

#[cfg(feature = "app-report")]
#[::unity2::methods]
impl Report {
    #[method(name = "MapStart", args = 0)]
    pub fn map_start() -> ();

    #[method(name = "MapComplete", args = 0)]
    pub fn map_complete() -> ();

    #[method(name = "Kizuna", args = 0)]
    pub fn kizuna() -> ();

    #[method(name = "AddBoughtItem", args = 1)]
    pub fn add_bought_item(item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "AddRefineItem", args = 2)]
    pub fn add_refine_item(item: crate::app::itemdata::ItemData, level: i32) -> ();

    #[method(name = "AddEvolveItem", args = 2)]
    pub fn add_evolve_item(
        prev: crate::app::itemdata::ItemData,
        next: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "AddUseItem", args = 1)]
    pub fn add_use_item(item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "AddPickupItem", args = 1)]
    pub fn add_pickup_item(item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "AddTalk", args = 1)]
    pub fn add_talk(person: crate::app::persondata::PersonData) -> ();

    #[method(name = "AddEquipSkillPool", args = 3)]
    pub fn add_equip_skill_pool(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        skill_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddEngageFull", args = 1)]
    pub fn add_engage_full(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AddEngageStart", args = 1)]
    pub fn add_engage_start(unit: crate::app::unit::Unit) -> ();

    #[method(name = "AddEngageAttack", args = 1)]
    pub fn add_engage_attack(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearRepoart", args = 0)]
    pub fn clear_repoart() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-report")]
impl Report {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Report),
                ::core::stringify!(new),
            )
        });
        <Self as IReportMethods>::ctor(this);
        this
    }
}
