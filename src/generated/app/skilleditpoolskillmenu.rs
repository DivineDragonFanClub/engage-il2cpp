
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditpoolskillmenu/SkillEditPoolSkillMenu.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditPoolSkillMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SkillEditPoolSkillMenu {
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[rename(name = "m_SkillList")]
    pub m_skill_list:
        crate::system::collections::generic::list_1::List_1<crate::app::skilldata::SkillData>,
}

#[cfg(feature = "app-skilleditpoolskillmenu")]
#[::unity2::methods]
impl SkillEditPoolSkillMenu {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::skilleditpoolskillmenucontent::SkillEditPoolSkillMenuContent,
    ) -> crate::app::skilleditpoolskillmenu::SkillEditPoolSkillMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skilleditpoolskillmenucontent::SkillEditPoolSkillMenuContent,
    ) -> ();

    #[method(name = "UpdateList", args = 0)]
    pub fn update_list(self) -> ();

    #[method(name = "GetSkill", args = 1)]
    pub fn get_skill(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "GetSelectSkill", args = 0)]
    pub fn get_select_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "IsEquipSkillExist", args = 0)]
    pub fn is_equip_skill_exist(self) -> bool;

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor(self, is_show: bool) -> ();

    #[method(name = "IsShowCursor", args = 0)]
    pub fn is_show_cursor(self) -> bool;

    #[method(name = "EnableInput", args = 1)]
    pub fn enable_input(self, is_enable: bool) -> ();

    #[method(name = "SetFirstSelection", args = 2)]
    pub fn set_first_selection(self, b_focus_enable: bool, b_update_info: bool) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "ChangeMenu", args = 0)]
    pub fn change_menu(self) -> ();
}

#[cfg(feature = "app-skilleditpoolskillmenu")]
impl SkillEditPoolSkillMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skilleditpoolskillmenucontent::SkillEditPoolSkillMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditPoolSkillMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditPoolSkillMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
