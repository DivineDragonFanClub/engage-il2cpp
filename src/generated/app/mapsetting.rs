
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsetting/MapSetting.md")))]
#[::unity2::class(namespace = "App", name = "MapSetting")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapsetting :: MapSetting >)]
pub struct MapSetting {
    #[rename(name = "m_MapTerrain")]
    pub m_map_terrain: crate::app::mapterrain::MapTerrain,
    #[rename(name = "m_ObjectList")]
    pub m_object_list:
        crate::system::collections::generic::list_1::List_1<crate::app::mapobject::MapObject>,
    #[rename(name = "m_ObjectDictionary")]
    pub m_object_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::mapobject::MapObject,
    >,
    #[static_field]
    #[rename(name = "s_BackupList")]
    pub s_backup_list: crate::app::mapbackuplist::MapBackupList,
    #[rename(name = "m_MapDevelop")]
    pub m_map_develop: crate::app::mapterrain::MapTerrain,
}

#[cfg(feature = "app-mapsetting")]
#[::unity2::methods]
impl MapSetting {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "FindMapObjectFromName", args = 1)]
    pub fn find_map_object_from_name(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::app::mapobject::MapObject;

    #[method(name = "FindMapObjectFromKey", args = 1)]
    pub fn find_map_object_from_key(
        self,
        key: ::unity2::Il2CppString,
    ) -> crate::app::mapobject::MapObject;

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Inactive", args = 0)]
    pub fn inactive(self) -> ();

    #[method(name = "ChangedActiveScene", args = 2)]
    pub fn changed_active_scene(
        self,
        current: crate::unity_engine::scene_management::scene::Scene,
        next: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "CommitMapObject", args = 0)]
    pub fn commit_map_object(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Resume", args = 0)]
    pub fn resume() -> ();

    #[method(name = "GetObjectList", args = 0)]
    pub fn get_object_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::mapobject::MapObject>;

    #[method(name = "UpdateLODGroup", args = 0)]
    pub fn update_lod_group(self) -> ();

    #[method(name = "get_MapTerrain", args = 0)]
    pub fn get_map_terrain() -> crate::app::mapterrain::MapTerrain;

    #[method(name = "get_Width", args = 0)]
    pub fn get_width() -> i32;

    #[method(name = "get_Height", args = 0)]
    pub fn get_height() -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsetting")]
impl MapSetting {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSetting),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSettingMethods>::ctor(this);
        this
    }
}
