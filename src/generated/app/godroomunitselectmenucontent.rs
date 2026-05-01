
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomunitselectmenucontent/GodRoomUnitSelectMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomUnitSelectMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct GodRoomUnitSelectMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_statusWindow")]
    pub m_status_window: crate::app::shopunitselectstatus::ShopUnitSelectStatus,
    #[rename(name = "m_godInfo")]
    pub m_god_info: crate::app::godroomgodinfosetter::GodRoomGodInfoSetter,
    #[rename(name = "m_GodListActive")]
    pub m_god_list_active:
        crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
}

#[cfg(feature = "app-godroomunitselectmenucontent")]
#[::unity2::methods]
impl GodRoomUnitSelectMenuContent {
    #[method(name = "GetGodListActive", args = 0)]
    pub fn get_god_list_active(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::godroomunitselectmenucontent::GodRoomUnitSelectMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(
        content: crate::app::godroomunitselectmenucontent::GodRoomUnitSelectMenuContent,
    ) -> ();

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "OpenStatus", args = 0)]
    pub fn open_status(self) -> ();

    #[method(name = "CloseStatus", args = 0)]
    pub fn close_status(self) -> ();

    #[method(name = "SetUnitStatus", args = 1)]
    pub fn set_unit_status(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "OnHelp", args = 1)]
    pub fn on_help(self, parent: crate::app::procinst::ProcInst) -> ();

    #[method(name = "UpdateGodInfo", args = 1)]
    pub fn update_god_info(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godroomunitselectmenucontent")]
impl GodRoomUnitSelectMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomUnitSelectMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomUnitSelectMenuContentMethods>::ctor(this);
        this
    }
}
