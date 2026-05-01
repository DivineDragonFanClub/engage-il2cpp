
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditequipskillmenu/SkillEditEquipSkillMenu.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditEquipSkillMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SkillEditEquipSkillMenu {}

#[cfg(feature = "app-skilleditequipskillmenu")]
#[::unity2::methods]
impl SkillEditEquipSkillMenu {
    #[method(name = "Create", args = 3)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        menu_content: crate::app::skilleditequipskillmenucontent::SkillEditEquipSkillMenuContent,
    ) -> crate::app::skilleditequipskillmenu::SkillEditEquipSkillMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skilleditequipskillmenucontent::SkillEditEquipSkillMenuContent,
    ) -> ();

    #[method(name = "GetSelectSkill", args = 0)]
    pub fn get_select_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor(self, is_show: bool) -> ();

    #[method(name = "IsShowCursor", args = 0)]
    pub fn is_show_cursor(self) -> bool;

    #[method(name = "EnableInput", args = 1)]
    pub fn enable_input(self, is_enable: bool) -> ();

    #[method(name = "SetFirstSelection", args = 1)]
    pub fn set_first_selection(self, b_update_info: bool) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "ChangeMenu", args = 0)]
    pub fn change_menu(self) -> ();

    #[method(name = "SetUnitName", args = 1)]
    pub fn set_unit_name(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetSkillNgActive", args = 1)]
    pub fn set_skill_ng_active(self, is_active: bool) -> ();
}

#[cfg(feature = "app-skilleditequipskillmenu")]
impl SkillEditEquipSkillMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skilleditequipskillmenucontent::SkillEditEquipSkillMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditEquipSkillMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditEquipSkillMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
