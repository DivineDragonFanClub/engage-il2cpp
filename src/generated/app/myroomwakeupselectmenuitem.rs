
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectmenuitem/MyRoomWakeupSelectMenuItem_SimpleFade.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectMenuItem.SimpleFade")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MyRoomWakeupSelectMenuItem_SimpleFade {}

#[cfg(feature = "app-myroomwakeupselectmenuitem")]
#[::unity2::methods]
impl MyRoomWakeupSelectMenuItem_SimpleFade {
    #[method(name = "CrateBlackOutBind", args = 1)]
    pub fn crate_black_out_bind(parent: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "DeleteParent", args = 0)]
    pub fn delete_parent(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomwakeupselectmenuitem")]
impl MyRoomWakeupSelectMenuItem_SimpleFade {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectMenuItem_SimpleFade),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectMenuItem_SimpleFadeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectmenuitem/MyRoomWakeupSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomWakeupSelectMenuItem {
    #[rename(name = "m_UnitL")]
    pub m_unit_l: crate::app::unit::Unit,
    #[rename(name = "m_UnitR")]
    pub m_unit_r: crate::app::unit::Unit,
}

#[cfg(feature = "app-myroomwakeupselectmenuitem")]
#[::unity2::methods]
impl MyRoomWakeupSelectMenuItem {
    #[method(name = "get_IsSelected", args = 0)]
    pub fn get_is_selected(self) -> bool;

    #[method(name = "set_IsSelected", args = 1)]
    pub fn set_is_selected(self, value: bool) -> ();

    #[method(name = "get_IsTalk", args = 0)]
    pub fn get_is_talk(self) -> bool;

    #[method(name = "set_IsTalk", args = 1)]
    pub fn set_is_talk(self, value: bool) -> ();

    #[method(name = "get_Cursor", args = 0)]
    pub fn get_cursor(self)
        -> crate::app::myroomwakeupselectroot::MyRoomWakeupSelectRoot_CursorTop;

    #[method(name = "get_IsBlank", args = 0)]
    pub fn get_is_blank(self) -> bool;

    #[method(name = "get_IsAPlus", args = 0)]
    pub fn get_is_a_plus(self) -> bool;

    #[method(name = "GetCommandColor", args = 0)]
    pub fn get_command_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "CanStart", args = 2)]
    pub fn can_start(
        self,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> bool;

    #[method(name = "GetSortOrder", args = 0)]
    pub fn get_sort_order(self) -> i32;

    #[method(name = "GetOpenCount", args = 0)]
    pub fn get_open_count(self) -> i32;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit_l: crate::app::unit::Unit, unit_r: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateTalk", args = 0)]
    pub fn update_talk(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "StartRankSelect", args = 2)]
    pub fn start_rank_select(
        self,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnitL", args = 0)]
    pub fn get_unit_l(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitR", args = 0)]
    pub fn get_unit_r(self) -> crate::app::unit::Unit;
}

#[cfg(feature = "app-myroomwakeupselectmenuitem")]
impl MyRoomWakeupSelectMenuItem {
    pub fn new(unit_l: crate::app::unit::Unit, unit_r: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectMenuItemMethods>::ctor(this, unit_l, unit_r);
        this
    }
}
