
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectmenu/MyRoomWakeupSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomWakeupSelectMenu {
    #[rename(name = "IsRankPhase")]
    pub is_rank_phase: bool,
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
    #[rename(name = "m_OldLevel")]
    pub m_old_level: crate::app::reliancedata::RelianceData_Level,
    #[rename(name = "m_OldPattern")]
    pub m_old_pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
}

#[cfg(feature = "app-myroomwakeupselectmenu")]
#[::unity2::methods]
impl MyRoomWakeupSelectMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        select: i32,
        scroll: i32,
    ) -> crate::app::myroomwakeupselectmenu::MyRoomWakeupSelectMenu;

    #[method(name = "CreateDefaultDesc", args = 0)]
    pub fn create_default_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateMenuList", args = 0)]
    pub fn create_menu_list(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomwakeupselectmenucontent::MyRoomWakeupSelectMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetOldLevelAndPattern", args = 2)]
    pub fn set_old_level_and_pattern(
        self,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-myroomwakeupselectmenu")]
impl MyRoomWakeupSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomwakeupselectmenucontent::MyRoomWakeupSelectMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
