
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingrodselectmenucontent/FishingRodSelectMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "FishingRodSelectMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct FishingRodSelectMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
}

#[cfg(feature = "app-fishingrodselectmenucontent")]
#[::unity2::methods]
impl FishingRodSelectMenuContent {
    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(shop_title_bar: crate::app::shoptopmenucontent::ShopTopMenuContent) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishingrodselectmenucontent")]
impl FishingRodSelectMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingRodSelectMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingRodSelectMenuContentMethods>::ctor(this);
        this
    }
}
