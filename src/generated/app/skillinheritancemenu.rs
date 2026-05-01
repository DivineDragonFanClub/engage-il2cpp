
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillinheritancemenu/SkillInheritanceMenu.md")))]
#[::unity2::class(namespace = "App", name = "SkillInheritanceMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SkillInheritanceMenu {
    #[static_field]
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    #[rename(name = "m_GodListActive")]
    pub m_god_list_active:
        crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
    #[rename(name = "m_GodIndex")]
    pub m_god_index: i32,
}

#[cfg(feature = "app-skillinheritancemenu")]
#[::unity2::methods]
impl SkillInheritanceMenu {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit() -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(value: crate::app::unit::Unit) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        god_list: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
        decide_event_handler: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "CreateMenuItemList", args = 1)]
    pub fn create_menu_item_list(
        god: crate::app::godunit::GodUnit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skillinheritancemenucontent::SkillInheritanceMenuContent,
        god_list: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    ) -> ();

    #[method(name = "SetupMenuSkill", args = 0)]
    pub fn setup_menu_skill(self) -> ();

    #[method(name = "UpdateSkillPoint", args = 0)]
    pub fn update_skill_point(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "GetGod", args = 0)]
    pub fn get_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "GetGod", args = 1)]
    pub fn get_god_2(self, index: i32) -> crate::app::godunit::GodUnit;

    #[method(name = "GetGodNum", args = 0)]
    pub fn get_god_num(self) -> i32;

    #[method(name = "SetSkillHelp", args = 2)]
    pub fn set_skill_help(self, skill: crate::app::skilldata::SkillData, level: i32) -> ();

    #[method(name = "GetSelectGodIndex", args = 0)]
    pub fn get_select_god_index(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-skillinheritancemenu")]
impl SkillInheritanceMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::skillinheritancemenucontent::SkillInheritanceMenuContent,
        god_list: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillInheritanceMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillInheritanceMenuMethods>::ctor(this, menu_item_list, menu_content, god_list);
        this
    }
}
