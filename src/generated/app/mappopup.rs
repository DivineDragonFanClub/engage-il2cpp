
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappopup/MapPopup.md")))]
#[::unity2::class(namespace = "App", name = "MapPopup")]
#[parent(crate::system::object::Object)]
pub struct MapPopup {}

#[cfg(feature = "app-mappopup")]
#[::unity2::methods]
impl MapPopup {
    #[method(name = "GetPopupPos", args = 1)]
    pub fn get_popup_pos(unit: crate::app::unit::Unit) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetPopupPos", args = 1)]
    pub fn get_popup_pos_2(
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetRoot", args = 0)]
    pub fn get_root() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PlayDamage", args = 3)]
    pub fn play_damage(
        side: crate::app::battleinfoside::BattleInfoSide,
        value: i32,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> ();

    #[method(name = "PlayDamage", args = 2)]
    pub fn play_damage_2(unit: crate::app::unit::Unit, value: i32) -> ();

    #[method(name = "PlayHeal", args = 2)]
    pub fn play_heal(unit: crate::app::unit::Unit, value: i32) -> ();

    #[method(name = "PlayHeal", args = 2)]
    pub fn play_heal_2(pos: crate::unity_engine::vector3::Vector3, value: i32) -> ();

    #[method(name = "PlayDamage", args = 2)]
    pub fn play_damage_3(pos: crate::unity_engine::vector3::Vector3, value: i32) -> ();

    #[method(name = "PlayCritical", args = 2)]
    pub fn play_critical(pos: crate::unity_engine::vector3::Vector3, value: i32) -> ();

    #[method(name = "PlayChainAttack", args = 2)]
    pub fn play_chain_attack(pos: crate::unity_engine::vector3::Vector3, value: i32) -> ();

    #[method(name = "PlaySkill", args = 2)]
    pub fn play_skill(unit: crate::app::unit::Unit, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "PlayText", args = 3)]
    pub fn play_text(
        pos: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color::Color,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PlayPopup", args = 2)]
    pub fn play_popup(
        side: crate::app::battleinfoside::BattleInfoSide,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PlayPopup", args = 2)]
    pub fn play_popup_2(unit: crate::app::unit::Unit, name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayPopup", args = 2)]
    pub fn play_popup_3(
        pos: crate::unity_engine::vector3::Vector3,
        name: ::unity2::Il2CppString,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mappopup")]
impl MapPopup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPopup),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPopupMethods>::ctor(this);
        this
    }
}
