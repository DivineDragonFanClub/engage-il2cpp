
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenucontent/SolanelInfoMenuContent_SolanelUnit.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuContent.SolanelUnit")]
#[parent(crate::system::object::Object)]
pub struct SolanelInfoMenuContent_SolanelUnit {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TalkIcon")]
    pub m_talk_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-solanelinfomenucontent")]
#[::unity2::methods]
impl SolanelInfoMenuContent_SolanelUnit {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetUnit", args = 2)]
    pub fn set_unit(
        self,
        unit: crate::app::unit::Unit,
        hub_access_data: crate::app::hubaccessdata::HubAccessData,
    ) -> ();

    #[method(name = "SetGodUnit", args = 2)]
    pub fn set_god_unit(
        self,
        god: crate::app::goddata::GodData,
        hub_access_data: crate::app::hubaccessdata::HubAccessData,
    ) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetTalkIcon", args = 1)]
    pub fn set_talk_icon(self, hub_access_data: crate::app::hubaccessdata::HubAccessData) -> ();
}

#[cfg(feature = "app-solanelinfomenucontent")]
impl SolanelInfoMenuContent_SolanelUnit {
    pub fn new(root: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuContent_SolanelUnit),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuContent_SolanelUnitMethods>::ctor(this, root);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenucontent/SolanelInfoMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct SolanelInfoMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SpriteAtlasPath")]
    pub sprite_atlas_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_SpriteAtlasManager")]
    pub s_sprite_atlas_manager: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[rename(name = "m_MapArrow")]
    pub m_map_arrow: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MapArrowMoveFrame")]
    pub m_map_arrow_move_frame: f32,
    #[rename(name = "m_MapPoint")]
    pub m_map_point: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_AreaName")]
    pub m_area_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AreaHelp")]
    pub m_area_help: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AreaFastTravelObj")]
    pub m_area_fast_travel_obj: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_HelpFastTravelList")]
    pub m_help_fast_travel_list: crate::system::collections::generic::list_1::List_1<
        crate::app::solanelinfomenucontent::SolanelInfoMenuContent_HelpFastTravel,
    >,
    #[rename(name = "m_UnitListObj")]
    pub m_unit_list_obj: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_SolanelUnitList")]
    pub m_solanel_unit_list: crate::system::collections::generic::list_1::List_1<
        crate::app::solanelinfomenucontent::SolanelInfoMenuContent_SolanelUnit,
    >,
    #[rename(name = "m_InfomationAbsentObj")]
    pub m_infomation_absent_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_InfomationListObj")]
    pub m_infomation_list_obj: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Pos")]
    pub m_pos: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_From")]
    pub m_from: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_To")]
    pub m_to: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_MoveTick")]
    pub m_move_tick: f32,
}

#[cfg(feature = "app-solanelinfomenucontent")]
#[::unity2::methods]
impl SolanelInfoMenuContent {
    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::solanelinfomenucontent::SolanelInfoMenuContent;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "GetSpriteAtlasManager", args = 0)]
    pub fn get_sprite_atlas_manager(self)
        -> crate::app::spriteatlasmanager_2::SpriteAtlasManager_2;

    #[method(name = "SetAreaDetail", args = 1)]
    pub fn set_area_detail(self, data: crate::app::hubareadata::HubAreaData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-solanelinfomenucontent")]
impl SolanelInfoMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenucontent/SolanelInfoMenuContent_Infomation.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuContent.Infomation")]
#[parent(crate::system::object::Object)]
pub struct SolanelInfoMenuContent_Infomation {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Message")]
    pub m_message: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-solanelinfomenucontent")]
#[::unity2::methods]
impl SolanelInfoMenuContent_Infomation {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 3)]
    pub fn show(
        self,
        sprite: crate::unity_engine::sprite::Sprite,
        title: ::unity2::Il2CppString,
        message: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-solanelinfomenucontent")]
impl SolanelInfoMenuContent_Infomation {
    pub fn new(root: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuContent_Infomation),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuContent_InfomationMethods>::ctor(this, root);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/solanelinfomenucontent/SolanelInfoMenuContent_HelpFastTravel.md")))]
#[::unity2::class(namespace = "App", name = "SolanelInfoMenuContent.HelpFastTravel")]
#[parent(crate::system::object::Object)]
pub struct SolanelInfoMenuContent_HelpFastTravel {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CheckDone")]
    pub m_check_done: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Value")]
    pub m_value: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-solanelinfomenucontent")]
#[::unity2::methods]
impl SolanelInfoMenuContent_HelpFastTravel {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "TrySetFastTravel", args = 2)]
    pub fn try_set_fast_travel(
        self,
        data: crate::app::hubfacilitydata::HubFacilityData,
        sprite: crate::unity_engine::sprite::Sprite,
    ) -> bool;

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetOneOnly", args = 1)]
    pub fn set_one_only(self, is_done: bool) -> ();

    #[method(name = "SetOneDone", args = 1)]
    pub fn set_one_done(self, is_done: bool) -> ();

    #[method(name = "SetCount", args = 1)]
    pub fn set_count(self, num: i32) -> ();
}

#[cfg(feature = "app-solanelinfomenucontent")]
impl SolanelInfoMenuContent_HelpFastTravel {
    pub fn new(root: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SolanelInfoMenuContent_HelpFastTravel),
                ::core::stringify!(new),
            )
        });
        <Self as ISolanelInfoMenuContent_HelpFastTravelMethods>::ctor(this, root);
        this
    }
}
