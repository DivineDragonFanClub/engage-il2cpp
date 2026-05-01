
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievementmenucontent/AchievementMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "AchievementMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct AchievementMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_activeIconColor")]
    pub m_active_icon_color: crate::unity_engine::color::Color,
    #[rename(name = "m_inactiveIconColor")]
    pub m_inactive_icon_color: crate::unity_engine::color::Color,
    #[rename(name = "m_categoryValue")]
    pub m_category_value: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    >,
    #[rename(name = "m_totalValue")]
    pub m_total_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_categoryIcon")]
    pub m_category_icon: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-achievementmenucontent")]
#[::unity2::methods]
impl AchievementMenuContent {
    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::achievementmenucontent::AchievementMenuContent;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "GetCategoryIconNum", args = 0)]
    pub fn get_category_icon_num(self) -> i32;

    #[method(name = "SetCategoryIconActive", args = 2)]
    pub fn set_category_icon_active(self, index: i32, is_active: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-achievementmenucontent")]
impl AchievementMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AchievementMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAchievementMenuContentMethods>::ctor(this);
        this
    }
}
