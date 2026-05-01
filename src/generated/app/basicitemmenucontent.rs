
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicitemmenucontent/BasicItemMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "BasicItemMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct BasicItemMenuContent {
    #[rename(name = "m_wdwItemHelp")]
    pub m_wdw_item_help: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_IsPerformanceOnly")]
    pub m_is_performance_only: bool,
}

#[cfg(feature = "app-basicitemmenucontent")]
#[::unity2::methods]
impl BasicItemMenuContent {
    #[method(name = "SetMenuTitle", args = 1)]
    pub fn set_menu_title(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "ChangeIsPerformanceOnly", args = 0)]
    pub fn change_is_performance_only(self) -> ();

    #[method(name = "SetDetail", args = 2)]
    pub fn set_detail(
        self,
        unit: crate::app::unit::Unit,
        selected_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "SetEnchantMode", args = 1)]
    pub fn set_enchant_mode(self, is_enchant: bool) -> ();

    #[method(name = "GetSubItemMenuRect", args = 0)]
    pub fn get_sub_item_menu_rect(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::basicitemmenucontent::BasicItemMenuContent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-basicitemmenucontent")]
impl BasicItemMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicItemMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicItemMenuContentMethods>::ctor(this);
        this
    }
}
