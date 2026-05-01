
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::basicskillmenuitem::BasicSkillMenuItem;
use crate::app::basicskillmenuitem::IBasicSkillMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditpoolskillmenuitem/SkillEditPoolSkillMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditPoolSkillMenuItem")]
#[parent(crate::app::basicskillmenuitem::BasicSkillMenuItem)]
pub struct SkillEditPoolSkillMenuItem {
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-skilleditpoolskillmenuitem")]
#[::unity2::methods]
impl SkillEditPoolSkillMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "EquipSkillSort", args = 2)]
    pub fn equip_skill_sort(
        self,
        manager: crate::app::sortieskilleditmanager::SortieSkillEditManager,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetMenuItemColor", args = 0)]
    pub fn set_menu_item_color(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "GetSkill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "IsActiveSkillIcon", args = 0)]
    pub fn is_active_skill_icon(self) -> bool;

    #[method(name = "GetSubText", args = 0)]
    pub fn get_sub_text(
        self,
    ) -> crate::app::basicskillmenuitemcontent::BasicSkillMenuItemContent_SubText;
}

#[cfg(feature = "app-skilleditpoolskillmenuitem")]
impl SkillEditPoolSkillMenuItem {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditPoolSkillMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditPoolSkillMenuItemMethods>::ctor(this, index);
        this
    }
}
