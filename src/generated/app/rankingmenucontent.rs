
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingmenucontent/RankingMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RankingMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RankingMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_chapterNo")]
    pub m_chapter_no: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_chapterTitle")]
    pub m_chapter_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-rankingmenucontent")]
#[::unity2::methods]
impl RankingMenuContent {
    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::rankingmenucontent::RankingMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: crate::app::rankingmenucontent::RankingMenuContent) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "SetCapter", args = 1)]
    pub fn set_capter(self, chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rankingmenucontent")]
impl RankingMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingMenuContentMethods>::ctor(this);
        this
    }
}
