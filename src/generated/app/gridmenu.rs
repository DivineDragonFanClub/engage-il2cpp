
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gridmenu/GridMenu.md")))]
#[::unity2::class(namespace = "App", name = "GridMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GridMenu {
    #[rename(name = "m_gridMenuContent")]
    pub m_grid_menu_content: crate::app::gridmenucontent::GridMenuContent,
    #[rename(name = "m_column")]
    pub m_column: i32,
}

#[cfg(feature = "app-gridmenu")]
#[::unity2::methods]
impl GridMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::gridmenucontent::GridMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "GetBuildRowNum", args = 0)]
    pub fn get_build_row_num(self) -> i32;

    #[method(name = "SetSelectIndex", args = 1)]
    pub fn set_select_index(self, index: i32) -> ();

    #[method(name = "RebuildInstant", args = 2)]
    pub fn rebuild_instant(
        self,
        menu_select: crate::app::basicmenuselect::BasicMenuSelect,
        display_index: i32,
    ) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "AdjustScrollIndex", args = 0)]
    pub fn adjust_scroll_index(self) -> ();

    #[method(name = "SetCellSize", args = 2)]
    pub fn set_cell_size(self, w: f32, h: f32) -> ();

    #[method(name = "SetCellW", args = 1)]
    pub fn set_cell_w(self, w: f32) -> ();

    #[method(name = "SetCellH", args = 1)]
    pub fn set_cell_h(self, h: f32) -> ();

    #[method(name = "GetCellSize", args = 0)]
    pub fn get_cell_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetCellW", args = 0)]
    pub fn get_cell_w(self) -> f32;

    #[method(name = "GetCellH", args = 0)]
    pub fn get_cell_h(self) -> f32;

    #[method(name = "SetColumnCount", args = 1)]
    pub fn set_column_count(self, count: i32) -> ();

    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetGridMenuContent", args = 0)]
    pub fn get_grid_menu_content(self) -> crate::app::gridmenucontent::GridMenuContent;
}

#[cfg(feature = "app-gridmenu")]
impl GridMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::gridmenucontent::GridMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GridMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGridMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
