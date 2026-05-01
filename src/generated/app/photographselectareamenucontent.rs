
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographselectareamenucontent/PhotographSelectAreaMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSelectAreaMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct PhotographSelectAreaMenuContent {
    #[static_field]
    #[rename(name = "s_MenuPrefabPath")]
    pub s_menu_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_AreaSpriteAtlasPath")]
    pub s_area_sprite_atlas_path: ::unity2::Il2CppString,
    #[rename(name = "m_AreaImage")]
    pub m_area_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SpotCount")]
    pub m_spot_count: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AreaSpriteAtlas")]
    pub m_area_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_AreaSprite")]
    pub m_area_sprite: crate::unity_engine::sprite::Sprite,
}

#[cfg(feature = "app-photographselectareamenucontent")]
#[::unity2::methods]
impl PhotographSelectAreaMenuContent {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "SetAreaData", args = 1)]
    pub fn set_area_data(self, area_data: crate::app::photographspotdata::PhotographSpotData)
        -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, idx: i32) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, idx: i32) -> f32;

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource() -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource() -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-photographselectareamenucontent")]
impl PhotographSelectAreaMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSelectAreaMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSelectAreaMenuContentMethods>::ctor(this);
        this
    }
}
