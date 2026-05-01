
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieutil/SortieUtil.md")))]
#[::unity2::class(namespace = "App", name = "SortieUtil")]
#[parent(crate::system::object::Object)]
pub struct SortieUtil {}

#[cfg(feature = "app-sortieutil")]
#[::unity2::methods]
impl SortieUtil {
    #[method(name = "GetUnitItem", args = 2)]
    pub fn get_unit_item(
        unit: crate::app::unit::Unit,
        index: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "CanStoreItem", args = 1)]
    pub fn can_store_item(unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "StoreAllItem", args = 1)]
    pub fn store_all_item(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "ExpendItem", args = 2)]
    pub fn expend_item(owner_unit: crate::app::unit::Unit, owner_item_index: i32) -> ();

    #[method(name = "GetFirstUnit", args = 0)]
    pub fn get_first_unit() -> crate::app::unit::Unit;

    #[method(name = "GetLastUnit", args = 0)]
    pub fn get_last_unit() -> crate::app::unit::Unit;

    #[method(name = "GetPrevUnit", args = 1)]
    pub fn get_prev_unit(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetNextUnit", args = 1)]
    pub fn get_next_unit(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetPrevUnitLoop", args = 1)]
    pub fn get_prev_unit_loop(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetNextUnitLoop", args = 1)]
    pub fn get_next_unit_loop(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetUnitCount", args = 0)]
    pub fn get_unit_count() -> i32;

    #[method(name = "GetSortieUnitCount", args = 0)]
    pub fn get_sortie_unit_count() -> i32;

    #[method(name = "GetSortieUnitMax", args = 0)]
    pub fn get_sortie_unit_max() -> i32;

    #[method(name = "IsPlayerUnitForRelay", args = 1)]
    pub fn is_player_unit_for_relay(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "RelocationUnitsBind", args = 1)]
    pub fn relocation_units_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CanStartBattle", args = 0)]
    pub fn can_start_battle() -> bool;

    #[method(name = "CanBackTo", args = 0)]
    pub fn can_back_to() -> bool;

    #[method(name = "IsUseLabelForNetworkBack", args = 0)]
    pub fn is_use_label_for_network_back() -> bool;

    #[method(name = "CannotStartBattleForRelay", args = 0)]
    pub fn cannot_start_battle_for_relay() -> bool;

    #[method(name = "NotifyCannotStartBattleForRelay", args = 1)]
    pub fn notify_cannot_start_battle_for_relay(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CanTalk", args = 0)]
    pub fn can_talk() -> bool;

    #[method(name = "CanTalk", args = 1)]
    pub fn can_talk_2(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsForbidStoring", args = 1)]
    pub fn is_forbid_storing(unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "SetCheckMark", args = 2)]
    pub fn set_check_mark(
        check: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetUnitFace", args = 2)]
    pub fn set_unit_face(
        face: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetSortieFaceColor", args = 2)]
    pub fn set_sortie_face_color(
        face: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetMapMenuFaceColor", args = 2)]
    pub fn set_map_menu_face_color(
        face: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetGodIconColor", args = 2)]
    pub fn set_god_icon_color(icon: crate::unity_engine::gameobject::GameObject, b_on: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieutil")]
impl SortieUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieUtil),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieUtilMethods>::ctor(this);
        this
    }
}
