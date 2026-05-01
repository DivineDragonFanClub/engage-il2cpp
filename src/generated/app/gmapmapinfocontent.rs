
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmapinfocontent/GmapMapInfoContent_ItemInfo.md")))]
#[::unity2::class(namespace = "App", name = "GmapMapInfoContent.ItemInfo")]
#[parent(crate::system::object::Object)]
pub struct GmapMapInfoContent_ItemInfo {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Sub")]
    pub m_sub: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-gmapmapinfocontent")]
#[::unity2::methods]
impl GmapMapInfoContent_ItemInfo {
    #[method(name = "SetupObj", args = 1)]
    pub fn setup_obj(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetName", args = 1)]
    pub fn set_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, icon_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetSubHide", args = 0)]
    pub fn set_sub_hide(self) -> ();

    #[method(name = "SetArrow", args = 0)]
    pub fn set_arrow(self) -> ();

    #[method(name = "SetIsCapture", args = 2)]
    pub fn set_is_capture(self, is_capture: bool, is_contains_lv1: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmapinfocontent")]
impl GmapMapInfoContent_ItemInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMapInfoContent_ItemInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMapInfoContent_ItemInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmapinfocontent/GmapMapInfoContent.md")))]
#[::unity2::class(namespace = "App", name = "GmapMapInfoContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GmapMapInfoContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SpriteAtlasPath")]
    pub sprite_atlas_path: ::unity2::Il2CppString,
    #[rename(name = "m_SpriteAtlas")]
    pub m_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_MapInfoAnim")]
    pub m_map_info_anim: crate::unity_engine::animator::Animator,
    #[rename(name = "m_MapInfoImage")]
    pub m_map_info_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MapInfoChapterObj")]
    pub m_map_info_chapter_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoTitle")]
    pub m_map_info_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MapInfoIconExpObj")]
    pub m_map_info_icon_exp_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoIconCoinObj")]
    pub m_map_info_icon_coin_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoIconEncountObj")]
    pub m_map_info_icon_encount_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoMessageRootObj")]
    pub m_map_info_message_root_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoMessageStoryObj")]
    pub m_map_info_message_story_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoMessageEncountObj")]
    pub m_map_info_message_encount_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoLvRootObj")]
    pub m_map_info_lv_root_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoLvObj")]
    pub m_map_info_lv_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapInfoCountryName")]
    pub m_map_info_country_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MapInfoCountryLv")]
    pub m_map_info_country_lv: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ItemMaterialList")]
    pub m_item_material_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_ItemFoodList")]
    pub m_item_food_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_ItemAnimalList")]
    pub m_item_animal_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_ItemInfoMaterialList")]
    pub m_item_info_material_list: crate::system::collections::generic::list_1::List_1<
        crate::app::gmapmapinfocontent::GmapMapInfoContent_ItemInfo,
    >,
    #[rename(name = "m_ItemInfoFoodList")]
    pub m_item_info_food_list: crate::system::collections::generic::list_1::List_1<
        crate::app::gmapmapinfocontent::GmapMapInfoContent_ItemInfo,
    >,
    #[rename(name = "m_ItemInfoAnimalList")]
    pub m_item_info_animal_list: crate::system::collections::generic::list_1::List_1<
        crate::app::gmapmapinfocontent::GmapMapInfoContent_ItemInfo,
    >,
    #[rename(name = "m_MapInfoSprite")]
    pub m_map_info_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_EncountInfo")]
    pub m_encount_info: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EncountInfoAnim")]
    pub m_encount_info_anim: crate::unity_engine::animator::Animator,
    #[rename(name = "m_MainTitle")]
    pub m_main_title: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GodTitle")]
    pub m_god_title: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EvilTitle")]
    pub m_evil_title: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MainValue")]
    pub m_main_value: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GodValue")]
    pub m_god_value: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EvilValue")]
    pub m_evil_value: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MainNormal")]
    pub m_main_normal: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MainGoldRare")]
    pub m_main_gold_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MainExpRare")]
    pub m_main_exp_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodNormal")]
    pub m_god_normal: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodGoldRare")]
    pub m_god_gold_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodExpRare")]
    pub m_god_exp_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EvilNormal")]
    pub m_evil_normal: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EvilGoldRare")]
    pub m_evil_gold_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EvilExpRare")]
    pub m_evil_exp_rare: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-gmapmapinfocontent")]
#[::unity2::methods]
impl GmapMapInfoContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::gmapmapinfocontent::GmapMapInfoContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: crate::app::gmapmapinfocontent::GmapMapInfoContent) -> ();

    #[method(name = "initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "OpenMapMini", args = 0)]
    pub fn open_map_mini(self) -> ();

    #[method(name = "OpenMapDetails", args = 0)]
    pub fn open_map_details(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "SetUpEncountInfo", args = 0)]
    pub fn set_up_encount_info(self) -> ();

    #[method(name = "OpenEncountInfo", args = 0)]
    pub fn open_encount_info(self) -> ();

    #[method(name = "CloseEncountInfo", args = 0)]
    pub fn close_encount_info(self) -> ();

    #[method(name = "SetMapInfo", args = 1)]
    pub fn set_map_info(self, gmap_spot: crate::app::gmapspot::GmapSpot) -> ();

    #[method(name = "SetItem", args = 4)]
    pub fn set_item(
        self,
        prefixless_cid: ::unity2::Il2CppString,
        nation_data: crate::app::hubnationdata::HubNationData,
        iscomplete: bool,
        is_unknown: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapmapinfocontent")]
impl GmapMapInfoContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMapInfoContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMapInfoContentMethods>::ctor(this);
        this
    }
}
