
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditor/MapEditor.md")))]
#[::unity2::class(namespace = "App", name = "MapEditor")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapeditor :: MapEditor >)]
pub struct MapEditor {
    #[rename(name = "m_EditObjects")]
    pub m_edit_objects: ::unity2::Array<crate::app::mapeditor::MapEditor_EditObject>,
    #[rename(name = "m_CasualMapEditData")]
    pub m_casual_map_edit_data: crate::app::casualmap::CasualMap,
    #[rename(name = "m_Mesh")]
    pub m_mesh: crate::app::map::Map_CellMesh,
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::app::mapeditor::MapEditor_EditObject,
    >,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_IsEnableGrid")]
    pub m_is_enable_grid: bool,
    #[static_field]
    #[rename(name = "s_IsPlaySound")]
    pub s_is_play_sound: bool,
    #[static_field]
    #[rename(name = "s_MapObjects")]
    pub s_map_objects: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::mappos::MapPos,
        i32,
    >,
    #[static_field]
    #[rename(name = "s_MapObjectRotates")]
    pub s_map_object_rotates: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::mappos::MapPos,
        i32,
    >,
    #[static_field]
    #[rename(name = "CasualMapSlotMax")]
    pub casual_map_slot_max: i32,
}

#[cfg(feature = "app-mapeditor")]
#[::unity2::methods]
impl MapEditor {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "CalcKey", args = 1)]
    pub fn calc_key(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "FindObject", args = 1)]
    pub fn find_object(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindObject", args = 1)]
    pub fn find_object_2(self, key: i32) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindObject", args = 2)]
    pub fn find_object_3(self, x: i32, z: i32) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetTerrain", args = 1)]
    pub fn get_terrain(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetTerrain", args = 1)]
    pub fn get_terrain_2(self, key: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "ForEachObject", args = 1)]
    pub fn for_each_object(
        self,
        func: crate::system::action_1::Action_1<crate::unity_engine::gameobject::GameObject>,
    ) -> ();

    #[method(name = "get_SizeX", args = 0)]
    pub fn get_size_x(self) -> i32;

    #[method(name = "get_SizeZ", args = 0)]
    pub fn get_size_z(self) -> i32;

    #[method(name = "GetEditObjectCount", args = 1)]
    pub fn get_edit_object_count(self, key: i32) -> i32;

    #[method(name = "GetEditObjectCount", args = 1)]
    pub fn get_edit_object_count_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetCategoryCount", args = 1)]
    pub fn get_category_count(
        self,
        category: crate::app::mapeditorcategorydata::MapEditorCategoryData,
    ) -> i32;

    #[method(name = "ContainsSettableArea", args = 3)]
    pub fn contains_settable_area(self, x: i32, z: i32, is_opponent: bool) -> bool;

    #[method(name = "IsOpponentArea", args = 2)]
    pub fn is_opponent_area(self, x: i32, z: i32) -> bool;

    #[method(name = "CanCreateObject", args = 1)]
    pub fn can_create_object(
        self,
        category: crate::app::mapeditorcategorydata::MapEditorCategoryData,
    ) -> bool;

    #[method(name = "ShowGrid", args = 0)]
    pub fn show_grid(self) -> ();

    #[method(name = "HideGrid", args = 0)]
    pub fn hide_grid(self) -> ();

    #[method(name = "UpdateGrid", args = 0)]
    pub fn update_grid(self) -> ();

    #[method(name = "GetMapPosition", args = 2)]
    pub fn get_map_position(cx: i32, cz: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TryCreateObject", args = 6)]
    pub fn try_create_object(
        self,
        x: i32,
        z: i32,
        obj: crate::app::mapeditorobjectdata::MapEditorObjectData,
        is_update: bool,
        is_record: bool,
        is_play_sound: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryCreateObject", args = 6)]
    pub fn try_create_object_2(
        self,
        x: i32,
        z: i32,
        key: i32,
        is_update: bool,
        is_record: bool,
        is_play_sound: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryDeleteObject", args = 5)]
    pub fn try_delete_object(
        self,
        x: i32,
        z: i32,
        is_update: bool,
        is_record: bool,
        is_play_sound: bool,
    ) -> bool;

    #[method(name = "TryReplaceObject", args = 3)]
    pub fn try_replace_object(
        self,
        x: i32,
        z: i32,
        obj_data: crate::app::mapeditorobjectdata::MapEditorObjectData,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CanReplace", args = 2)]
    pub fn can_replace(
        self,
        delete_obj: crate::unity_engine::gameobject::GameObject,
        create_obj: crate::app::mapeditorobjectdata::MapEditorObjectData,
    ) -> bool;

    #[method(name = "ClearObjectAll", args = 1)]
    pub fn clear_object_all(self, is_update: bool) -> ();

    #[method(name = "InstantiateObject", args = 3)]
    pub fn instantiate_object(
        self,
        put_object: crate::unity_engine::gameobject::GameObject,
        x: i32,
        z: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "DeleteObject", args = 1)]
    pub fn delete_object(self, put_object: crate::app::mapobject::MapObject) -> ();

    #[method(name = "DeleteObject", args = 1)]
    pub fn delete_object_2(self, put_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "TryRotateObject", args = 3)]
    pub fn try_rotate_object(self, x: i32, z: i32, is_reverse: bool) -> bool;

    #[method(name = "AddRotateObjectCommand", args = 4)]
    pub fn add_rotate_object_command(
        self,
        x: i32,
        z: i32,
        prev_rotate: i32,
        next_rotate: i32,
    ) -> ();

    #[method(name = "GetRotate", args = 2)]
    pub fn get_rotate(self, x: i32, z: i32) -> i32;

    #[method(name = "GetRotate", args = 1)]
    pub fn get_rotate_2(self, game_object: crate::unity_engine::gameobject::GameObject) -> i32;

    #[method(name = "SetRotate", args = 3)]
    pub fn set_rotate(self, x: i32, z: i32, rotate: i32) -> ();

    #[method(name = "SetRotate", args = 2)]
    pub fn set_rotate_2(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
        rotate: i32,
    ) -> ();

    #[method(name = "UpdateTerrain", args = 0)]
    pub fn update_terrain(self) -> ();

    #[method(name = "UpdateTerrain", args = 2)]
    pub fn update_terrain_2(self, pos_x: i32, pos_z: i32) -> ();

    #[method(name = "UpdateTerrainImpl", args = 0)]
    pub fn update_terrain_impl(self) -> ();

    #[method(name = "PlayPutSound", args = 1)]
    pub fn play_put_sound(object_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayDestroySound", args = 0)]
    pub fn play_destroy_sound() -> ();

    #[method(name = "SetPlaySound", args = 1)]
    pub fn set_play_sound(value: bool) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "TryCreateVersusObject", args = 5)]
    pub fn try_create_versus_object(
        self,
        x: i32,
        z: i32,
        key: i32,
        is_opponent: bool,
        is_update: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "ClearObjectOnSortie", args = 0)]
    pub fn clear_object_on_sortie(self) -> ();

    #[method(name = "SaveObjects", args = 0)]
    pub fn save_objects(self) -> ();

    #[method(name = "LoadObjects", args = 0)]
    pub fn load_objects(self) -> ();

    #[method(name = "LoadRotateAfterLoadObjects", args = 0)]
    pub fn load_rotate_after_load_objects(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SaveCasual", args = 1)]
    pub fn save_casual(self, stage: i32) -> ();

    #[method(name = "LoadCasual", args = 1)]
    pub fn load_casual(self, stage: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapeditor")]
impl MapEditor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditor),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditor/MapEditor_EditObject.md")))]
#[::unity2::class(namespace = "App", name = "MapEditor.EditObject")]
#[parent(crate::system::object::Object)]
pub struct MapEditor_EditObject {
    #[rename(name = "gameObject")]
    pub game_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mapeditor")]
#[::unity2::methods]
impl MapEditor_EditObject {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeditor")]
impl MapEditor_EditObject {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditor_EditObject),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditor_EditObjectMethods>::ctor(this);
        this
    }
}
