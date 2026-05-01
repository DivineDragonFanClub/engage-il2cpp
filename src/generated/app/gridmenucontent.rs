
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gridmenucontent/GridMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "GridMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct GridMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_gridLayoutGroup")]
    pub m_grid_layout_group: crate::unity_engine::ui::gridlayoutgroup::GridLayoutGroup,
}

#[cfg(feature = "app-gridmenucontent")]
#[::unity2::methods]
impl GridMenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CycleMenuItemContent", args = 2)]
    pub fn cycle_menu_item_content(self, is_forward: bool, cycle_count: i32) -> ();

    #[method(name = "GetLineHeightForScroll", args = 0)]
    pub fn get_line_height_for_scroll(self) -> f32;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "CalcColumn", args = 1)]
    pub fn calc_column(self, menu_item_index: i32) -> i32;

    #[method(name = "CalcRow", args = 1)]
    pub fn calc_row(self, menu_item_index: i32) -> i32;

    #[method(name = "CalcMenuItemIndex", args = 2)]
    pub fn calc_menu_item_index(self, row: i32, column: i32) -> i32;

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

    #[method(name = "UpdateCursorSize", args = 0)]
    pub fn update_cursor_size(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::gridmenucontent::GridMenuContent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gridmenucontent")]
impl GridMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GridMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGridMenuContentMethods>::ctor(this);
        this
    }
}
