
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::app::gridmenucontent::GridMenuContent;
use crate::app::gridmenucontent::IGridMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trademenucontent/TradeMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "TradeMenuContent")]
#[parent(crate::app::gridmenucontent::GridMenuContent)]
pub struct TradeMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LeftMenuItemBgName")]
    pub left_menu_item_bg_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RightMenuItemBgName")]
    pub right_menu_item_bg_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LeftNameBgName")]
    pub left_name_bg_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LeftNameTextName")]
    pub left_name_text_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RightNameBgName")]
    pub right_name_bg_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RightNameTextName")]
    pub right_name_text_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SelectBackCursorName")]
    pub select_back_cursor_name: ::unity2::Il2CppString,
    #[rename(name = "m_leftMenuItemBgImage")]
    pub m_left_menu_item_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_rightMenuItemBgImage")]
    pub m_right_menu_item_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_leftNameBgImage")]
    pub m_left_name_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_leftNameText")]
    pub m_left_name_text: crate::unity_engine::ui::text::Text,
    #[rename(name = "m_rightNameBgImage")]
    pub m_right_name_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_rightNameText")]
    pub m_right_name_text: crate::unity_engine::ui::text::Text,
    #[rename(name = "m_objSelectBackCursor")]
    pub m_obj_select_back_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_selectBackCursorImage")]
    pub m_select_back_cursor_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-trademenucontent")]
#[::unity2::methods]
impl TradeMenuContent {
    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "BuildMaterial", args = 0)]
    pub fn build_material(self) -> ();

    #[method(name = "UpdateCursorSize", args = 0)]
    pub fn update_cursor_size(self) -> ();

    #[method(name = "SetLeftName", args = 1)]
    pub fn set_left_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetRightName", args = 1)]
    pub fn set_right_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetSelectCursorPos", args = 1)]
    pub fn set_select_cursor_pos(self, menu_item_index: i32) -> ();

    #[method(name = "ShowSelectCursor", args = 0)]
    pub fn show_select_cursor(self) -> ();

    #[method(name = "HideSelectCursor", args = 0)]
    pub fn hide_select_cursor(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::trademenucontent::TradeMenuContent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-trademenucontent")]
impl TradeMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TradeMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ITradeMenuContentMethods>::ctor(this);
        this
    }
}
