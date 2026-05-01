
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysortie/RelaySortie.md")))]
#[::unity2::class(namespace = "App", name = "RelaySortie")]
#[parent(crate::system::object::Object)]
pub struct RelaySortie {
    #[static_field]
    #[rename(name = "NearbyFriendsThreshold")]
    pub nearby_friends_threshold: i32,
    #[rename(name = "m_MaxTotalCount")]
    pub m_max_total_count: i32,
    #[rename(name = "m_ReqMyCount")]
    pub m_req_my_count: i32,
    #[rename(name = "m_ReqTotalCountNoClamp")]
    pub m_req_total_count_no_clamp: i32,
    #[rename(name = "m_ReqTotalCount")]
    pub m_req_total_count: i32,
    #[rename(name = "m_TotalCount")]
    pub m_total_count: i32,
    #[rename(name = "m_MyCount")]
    pub m_my_count: i32,
    #[rename(name = "m_OtherCount")]
    pub m_other_count: i32,
    #[rename(name = "m_OtherPids")]
    pub m_other_pids:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
    #[rename(name = "m_OtherPos")]
    pub m_other_pos: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::app::mappos::MapPos,
    >,
    #[rename(name = "m_NearbyFriendsScores")]
    pub m_nearby_friends_scores: ::unity2::Array<i32>,
}

#[cfg(feature = "app-relaysortie")]
#[::unity2::methods]
impl RelaySortie {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "CountUnits", args = 0)]
    pub fn count_units(self) -> ();

    #[method(name = "SetupUnits", args = 0)]
    pub fn setup_units(self) -> ();

    #[method(name = "CreateSortiePosition", args = 0)]
    pub fn create_sortie_position(self) -> ();

    #[method(name = "PrerelocationUnits", args = 0)]
    pub fn prerelocation_units(self) -> ();

    #[method(name = "RelocationUnitForTakeOver", args = 1)]
    pub fn relocation_unit_for_take_over(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "FindEmptySortiePosition", args = 2)]
    pub fn find_empty_sortie_position(self, x: i32, z: i32) -> bool;

    #[method(name = "CountEmptySortiePosition", args = 0)]
    pub fn count_empty_sortie_position(self) -> i32;

    #[method(name = "FindSortiePositionNearbyFriends", args = 2)]
    pub fn find_sortie_position_nearby_friends(self, x: i32, z: i32) -> bool;

    #[method(name = "IsIgnore", args = 1)]
    pub fn is_ignore(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsIgnoreForTroop", args = 2)]
    pub fn is_ignore_for_troop(self, unit: crate::app::unit::Unit, is_in_battle_map: bool) -> bool;

    #[method(name = "CanSortie", args = 1)]
    pub fn can_sortie(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanToggle", args = 1)]
    pub fn can_toggle(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Toggle", args = 1)]
    pub fn toggle(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CanStartBattle", args = 0)]
    pub fn can_start_battle(self) -> bool;

    #[method(name = "CanBackTo", args = 0)]
    pub fn can_back_to(self) -> bool;

    #[method(name = "get_MaxTotalCount", args = 0)]
    pub fn get_max_total_count(self) -> i32;

    #[method(name = "get_ReqMyCount", args = 0)]
    pub fn get_req_my_count(self) -> i32;

    #[method(name = "get_ReqTotalCountNoClamp", args = 0)]
    pub fn get_req_total_count_no_clamp(self) -> i32;

    #[method(name = "get_ReqTotalCount", args = 0)]
    pub fn get_req_total_count(self) -> i32;

    #[method(name = "get_TotalCount", args = 0)]
    pub fn get_total_count(self) -> i32;

    #[method(name = "set_TotalCount", args = 1)]
    pub fn set_total_count(self, value: i32) -> ();

    #[method(name = "get_MyCount", args = 0)]
    pub fn get_my_count(self) -> i32;

    #[method(name = "set_MyCount", args = 1)]
    pub fn set_my_count(self, value: i32) -> ();

    #[method(name = "get_OtherCount", args = 0)]
    pub fn get_other_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaysortie")]
impl RelaySortie {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelaySortie),
                ::core::stringify!(new),
            )
        });
        <Self as IRelaySortieMethods>::ctor(this);
        this
    }
}
