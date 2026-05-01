
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicskillmenuitem/BasicSkillMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "BasicSkillMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct BasicSkillMenuItem {}

#[cfg(feature = "app-basicskillmenuitem")]
#[::unity2::methods]
impl BasicSkillMenuItem {
    #[method(name = "GetSkill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "IsActiveSkillIcon", args = 0)]
    pub fn is_active_skill_icon(self) -> bool;

    #[method(name = "GetSubText", args = 0)]
    pub fn get_sub_text(
        self,
    ) -> crate::app::basicskillmenuitemcontent::BasicSkillMenuItemContent_SubText;

    #[method(name = "GetBlankText", args = 0)]
    pub fn get_blank_text(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-basicskillmenuitem")]
impl BasicSkillMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicSkillMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicSkillMenuItemMethods>::ctor(this);
        this
    }
}
