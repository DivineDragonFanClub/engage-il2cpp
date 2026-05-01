
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bondlevelupskill/BondLevelUpSkill.md")))]
#[::unity2::class(namespace = "App", name = "BondLevelUpSkill")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BondLevelUpSkill {
    #[rename(name = "m_Skill")]
    pub m_skill: crate::app::ringlistskillmenuitemcontent::RingListSkillMenuItemContent,
    #[rename(name = "m_SkillDetailHelp")]
    pub m_skill_detail_help: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillDetailSetter")]
    pub m_skill_detail_setter: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Anim")]
    pub m_anim: crate::unity_engine::animator::Animator,
    #[rename(name = "m_NextMenuItem")]
    pub m_next_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    #[rename(name = "m_NextGod")]
    pub m_next_god: crate::app::godunit::GodUnit,
}

#[cfg(feature = "app-bondlevelupskill")]
#[::unity2::methods]
impl BondLevelUpSkill {
    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetSkill", args = 2)]
    pub fn set_skill(
        self,
        menu_item: crate::app::basicmenuitem::BasicMenuItem,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "SetNext", args = 2)]
    pub fn set_next(
        self,
        menu_item: crate::app::basicmenuitem::BasicMenuItem,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "IsPlayAnim", args = 0)]
    pub fn is_play_anim(self) -> bool;

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-bondlevelupskill")]
impl BondLevelUpSkill {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BondLevelUpSkill),
                ::core::stringify!(new),
            )
        });
        <Self as IBondLevelUpSkillMethods>::ctor(this);
        this
    }
}
