
use crate::app::pool::IPool_List_1;
use crate::app::pool::Pool_List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlescenelist/BattleSceneList.md")))]
#[::unity2::class(namespace = "App", name = "BattleSceneList")]
# [parent (crate :: app :: pool :: Pool_List_1 < crate :: app :: battlescene :: BattleScene >)]
pub struct BattleSceneList {
    #[static_field]
    #[rename(name = "MaxScene")]
    pub max_scene: i32,
    #[static_field]
    #[rename(name = "MaxTimes")]
    pub max_times: i32,
    #[rename(name = "m_Info")]
    pub m_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-battlescenelist")]
#[::unity2::methods]
impl BattleSceneList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "IsEntry", args = 1)]
    pub fn is_entry(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "GetCount", args = 1)]
    pub fn get_count(self, kind: crate::app::battlescene::BattleScene_Kind) -> i32;

    #[method(name = "FindNext", args = 2)]
    pub fn find_next(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        index: i32,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "FindPrev", args = 2)]
    pub fn find_prev(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        index: i32,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "GetCount", args = 2)]
    pub fn get_count_2(
        self,
        side: crate::app::battleside::BattleSide_Type,
        kind: crate::app::battlescene::BattleScene_Kind,
    ) -> i32;

    #[method(name = "HasGiveSkill", args = 2)]
    pub fn has_give_skill(
        self,
        side: crate::app::battleside::BattleSide_Type,
        give: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "HasChain", args = 0)]
    pub fn has_chain(self) -> bool;

    #[method(name = "GetAttackCount", args = 2)]
    pub fn get_attack_count(
        self,
        current: crate::app::battleside::BattleSide_Type,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> i32;

    #[method(name = "GetGiveSkillkCount", args = 2)]
    pub fn get_give_skillk_count(
        self,
        current: crate::app::battleside::BattleSide_Type,
        skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "CanSkillCount", args = 3)]
    pub fn can_skill_count(
        scene: crate::app::battlescene::BattleScene,
        target: crate::app::battleside::BattleSide_Type,
        reversed: bool,
    ) -> bool;

    #[method(name = "GetActiveSkillkCount", args = 1)]
    pub fn get_active_skillk_count(self, current: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "GetReciveActiveSkillkCount", args = 1)]
    pub fn get_recive_active_skillk_count(
        self,
        reverse: crate::app::battleside::BattleSide_Type,
    ) -> i32;

    #[method(name = "GetReciveAttackCount", args = 2)]
    pub fn get_recive_attack_count(
        self,
        reverse: crate::app::battleside::BattleSide_Type,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> i32;

    #[method(name = "Create", args = 1)]
    pub fn create(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "Create", args = 2)]
    pub fn create_2(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "Create", args = 3)]
    pub fn create_3(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        current: crate::app::battleinfoside::BattleInfoSide,
        target: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "Create", args = 2)]
    pub fn create_4(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        current: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "Create", args = 3)]
    pub fn create_5(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        current: crate::app::battleside::BattleSide_Type,
        target: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg(feature = "app-battlescenelist")]
impl BattleSceneList {
    pub fn new(info: crate::app::battleinfo::BattleInfo) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleSceneList),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleSceneListMethods>::ctor(this, info);
        this
    }
}
