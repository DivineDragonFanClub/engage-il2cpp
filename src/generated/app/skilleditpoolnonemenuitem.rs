
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::basicskillmenuitem::BasicSkillMenuItem;
use crate::app::basicskillmenuitem::IBasicSkillMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditpoolnonemenuitem/SkillEditPoolNoneMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditPoolNoneMenuItem")]
#[parent(crate::app::basicskillmenuitem::BasicSkillMenuItem)]
pub struct SkillEditPoolNoneMenuItem {}

#[cfg(feature = "app-skilleditpoolnonemenuitem")]
#[::unity2::methods]
impl SkillEditPoolNoneMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

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

#[cfg(feature = "app-skilleditpoolnonemenuitem")]
impl SkillEditPoolNoneMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditPoolNoneMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditPoolNoneMenuItemMethods>::ctor(this);
        this
    }
}
