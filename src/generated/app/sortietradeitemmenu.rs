
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortietradeitemmenu/SortieTradeItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieTradeItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SortieTradeItemMenu {
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_recieverUnit")]
    pub m_reciever_unit: crate::app::unit::Unit,
    #[rename(name = "m_firstSelect")]
    pub m_first_select: i32,
    #[rename(name = "m_secondSelect")]
    pub m_second_select: i32,
}

#[cfg(feature = "app-sortietradeitemmenu")]
#[::unity2::methods]
impl SortieTradeItemMenu {
    #[method(name = "get_m_CommonIndex", args = 0)]
    pub fn get_m_common_index(self) -> i32;

    #[method(name = "set_m_CommonIndex", args = 1)]
    pub fn set_m_common_index(self, value: i32) -> ();

    #[method(name = "Create", args = 5)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        reciever_unit: crate::app::unit::Unit,
        default_select: i32,
        menu_content: crate::app::sortietradeitemmenucontent::SortieTradeItemMenuContent,
    ) -> crate::app::sortietradeitemmenu::SortieTradeItemMenu;

    #[method(name = "CreateMenuItemList", args = 3)]
    pub fn create_menu_item_list(
        unit: crate::app::unit::Unit,
        reciever_unit: crate::app::unit::Unit,
        default_select: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortietradeitemmenucontent::SortieTradeItemMenuContent,
    ) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetRecieverUnit", args = 1)]
    pub fn set_reciever_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetSelectItem", args = 0)]
    pub fn get_select_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "ClearSelect", args = 0)]
    pub fn clear_select(self) -> ();

    #[method(name = "SetSelect", args = 1)]
    pub fn set_select(self, select_no: i32) -> ();

    #[method(name = "SetFirstSelectAtSelectIndex", args = 0)]
    pub fn set_first_select_at_select_index(self) -> ();

    #[method(name = "GetFirstSelect", args = 0)]
    pub fn get_first_select(self) -> i32;

    #[method(name = "GetSecondSelect", args = 0)]
    pub fn get_second_select(self) -> i32;

    #[method(name = "IsFirstSelect", args = 1)]
    pub fn is_first_select(
        self,
        menu_item: crate::app::sortietradeitemmenuitem::SortieTradeItemMenuItem,
    ) -> bool;

    #[method(name = "GetFirstSelectMenuItem", args = 0)]
    pub fn get_first_select_menu_item(
        self,
    ) -> crate::app::sortietradeitemmenuitem::SortieTradeItemMenuItem;

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor(self, is_show: bool) -> ();

    #[method(name = "SetInitialSelect", args = 0)]
    pub fn set_initial_select(self) -> ();

    #[method(name = "MoveFrontCursorFrom", args = 1)]
    pub fn move_front_cursor_from(self, another_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "SetSelectIndexAtFirstSelect", args = 0)]
    pub fn set_select_index_at_first_select(self) -> ();

    #[method(name = "SetSelectIndexEx", args = 1)]
    pub fn set_select_index_ex(self, common_index: i32) -> ();

    #[method(name = "SetSelectIndexForSecondSelect", args = 1)]
    pub fn set_select_index_for_second_select(self, another_index: i32) -> ();

    #[method(name = "EnableToSelectBlank", args = 1)]
    pub fn enable_to_select_blank(self, enabled: bool) -> ();

    #[method(name = "SetSelectableBlankToDisable", args = 1)]
    pub fn set_selectable_blank_to_disable(self, disabled: bool) -> ();

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;
}

#[cfg(feature = "app-sortietradeitemmenu")]
impl SortieTradeItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::sortietradeitemmenucontent::SortieTradeItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieTradeItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieTradeItemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
