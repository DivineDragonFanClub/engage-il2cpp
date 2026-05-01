
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillinheritancemenuitem/SkillInheritanceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SkillInheritanceMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct SkillInheritanceMenuItem {
    #[static_field]
    #[rename(name = "c_ReleaseLevel")]
    pub c_release_level: i32,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::system::action_1::Action_1<crate::app::unit::Unit>,
}

#[cfg(feature = "app-skillinheritancemenuitem")]
#[::unity2::methods]
impl SkillInheritanceMenuItem {
    #[method(name = "get_Skill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_Skill", args = 1)]
    pub fn set_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_SortId", args = 0)]
    pub fn get_sort_id(self) -> i32;

    #[method(name = "set_SortId", args = 1)]
    pub fn set_sort_id(self, value: i32) -> ();

    #[method(name = "get_SkillLevel", args = 0)]
    pub fn get_skill_level(self) -> i32;

    #[method(name = "set_SkillLevel", args = 1)]
    pub fn set_skill_level(self, value: i32) -> ();

    #[method(name = "get_SkillCost", args = 0)]
    pub fn get_skill_cost(self) -> i32;

    #[method(name = "set_SkillCost", args = 1)]
    pub fn set_skill_cost(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        sort_id: i32,
        skill: crate::app::skilldata::SkillData,
        skill_level: i32,
        decide_event_handler: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "IsInherited", args = 0)]
    pub fn is_inherited(self) -> bool;

    #[method(name = "IsEnoughLevel", args = 0)]
    pub fn is_enough_level(self) -> bool;

    #[method(name = "IsEnoughSp", args = 0)]
    pub fn is_enough_sp(self) -> bool;

    #[method(name = "ResetCost", args = 0)]
    pub fn reset_cost(self) -> ();

    #[method(name = "OnInherite", args = 0)]
    pub fn on_inherite(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetGod", args = 0)]
    pub fn get_god(self) -> crate::app::godunit::GodUnit;
}

#[cfg(feature = "app-skillinheritancemenuitem")]
impl SkillInheritanceMenuItem {
    pub fn new(
        sort_id: i32,
        skill: crate::app::skilldata::SkillData,
        skill_level: i32,
        decide_event_handler: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillInheritanceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillInheritanceMenuItemMethods>::ctor(
            this,
            sort_id,
            skill,
            skill_level,
            decide_event_handler,
        );
        this
    }
}
