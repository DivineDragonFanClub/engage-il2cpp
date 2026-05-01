
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrelianceselect/MyRoomRelianceSelect_MyRoomRelianceSelectItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomRelianceSelect.MyRoomRelianceSelectItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRelianceSelect_MyRoomRelianceSelectItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-myroomrelianceselect")]
#[::unity2::methods]
impl MyRoomRelianceSelect_MyRoomRelianceSelectItem {
    #[method(name = "get_IsTalk", args = 0)]
    pub fn get_is_talk(self) -> bool;

    #[method(name = "set_IsTalk", args = 1)]
    pub fn set_is_talk(self, value: bool) -> ();

    #[method(name = "get_IsBlank", args = 0)]
    pub fn get_is_blank(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetSortOrder", args = 0)]
    pub fn get_sort_order(self) -> i32;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateTalkState", args = 0)]
    pub fn update_talk_state(self) -> ();
}

#[cfg(feature = "app-myroomrelianceselect")]
impl MyRoomRelianceSelect_MyRoomRelianceSelectItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSelect_MyRoomRelianceSelectItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSelect_MyRoomRelianceSelectItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomrelianceselect/MyRoomRelianceSelect.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRelianceSelect")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomRelianceSelect {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-myroomrelianceselect")]
#[::unity2::methods]
impl MyRoomRelianceSelect {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetUnitList", args = 0)]
    pub fn get_unit_list(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>;

    #[method(name = "CreateMenuList", args = 0)]
    pub fn create_menu_list(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomrelianceselectcontent::MyRoomRelianceSelectContent,
    ) -> ();

    #[method(name = "RebuildInstantHide", args = 0)]
    pub fn rebuild_instant_hide(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "UpdateUnit", args = 1)]
    pub fn update_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "UpdateSelectMenuItem", args = 0)]
    pub fn update_select_menu_item(self) -> ();

    #[method(name = "ForceSelectUnit", args = 1)]
    pub fn force_select_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ForceUpdateMenuItem", args = 0)]
    pub fn force_update_menu_item(self) -> ();

    #[method(name = "NextSelect", args = 0)]
    pub fn next_select(self) -> ();

    #[method(name = "PrevSelect", args = 0)]
    pub fn prev_select(self) -> ();

    #[method(name = "GetCurrentUnit", args = 0)]
    pub fn get_current_unit(self) -> crate::app::unit::Unit;
}

#[cfg(feature = "app-myroomrelianceselect")]
impl MyRoomRelianceSelect {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomrelianceselectcontent::MyRoomRelianceSelectContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSelectMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
