
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapoverlap/MapOverlap_Data.md")))]
#[::unity2::class(namespace = "App", name = "MapOverlap.Data")]
#[parent(crate::system::object::Object)]
pub struct MapOverlap_Data {}

#[cfg(feature = "app-mapoverlap")]
#[::unity2::methods]
impl MapOverlap_Data {
    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_Hp", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "set_Hp", args = 1)]
    pub fn set_hp(self, value: i32) -> ();

    #[method(name = "get_Life", args = 0)]
    pub fn get_life(self) -> i32;

    #[method(name = "set_Life", args = 1)]
    pub fn set_life(self, value: i32) -> ();

    #[method(name = "get_Turn", args = 0)]
    pub fn get_turn(self) -> i32;

    #[method(name = "set_Turn", args = 1)]
    pub fn set_turn(self, value: i32) -> ();

    #[method(name = "get_Phase", args = 0)]
    pub fn get_phase(self) -> crate::app::force::Force_Type;

    #[method(name = "set_Phase", args = 1)]
    pub fn set_phase(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_Effect", args = 0)]
    pub fn get_effect(self) -> crate::app::resourceobject::ResourceObject;

    #[method(name = "set_Effect", args = 1)]
    pub fn set_effect(self, value: crate::app::resourceobject::ResourceObject) -> ();

    #[method(name = "get_Terrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "get_Age", args = 0)]
    pub fn get_age(self) -> i32;

    #[method(name = "get_MaxHp", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "IsSight", args = 0)]
    pub fn is_sight(self) -> bool;

    #[method(name = "CanBreakable", args = 0)]
    pub fn can_breakable(self) -> bool;

    #[method(name = "CanBreakable", args = 1)]
    pub fn can_breakable_2(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanBreakable", args = 1)]
    pub fn can_breakable_3(self, force: crate::app::force::Force_Type) -> bool;

    #[method(name = "get_Sight", args = 0)]
    pub fn get_sight(self) -> i32;

    #[method(name = "GetPosition", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetScale", args = 0)]
    pub fn get_scale(self) -> f32;

    #[method(name = "CreateEffect", args = 0)]
    pub fn create_effect(self) -> ();

    #[method(name = "UpdateEffect", args = 0)]
    pub fn update_effect(self) -> ();

    #[method(name = "DeleteEffect", args = 0)]
    pub fn delete_effect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapoverlap")]
impl MapOverlap_Data {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapOverlap_Data),
                ::core::stringify!(new),
            )
        });
        <Self as IMapOverlap_DataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapoverlap/MapOverlap.md")))]
#[::unity2::class(namespace = "App", name = "MapOverlap")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapoverlap :: MapOverlap >)]
pub struct MapOverlap {
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[rename(name = "m_Image")]
    pub m_image: crate::app::mapimagecorebyte::MapImageCoreByte,
    #[rename(name = "m_List")]
    pub m_list: crate::app::mapoverlap::MapOverlap_List,
}

#[cfg(feature = "app-mapoverlap")]
#[::unity2::methods]
impl MapOverlap {
    #[method(name = "SetImpl", args = 6)]
    pub fn set_impl(
        self,
        x: i32,
        z: i32,
        terrain: crate::app::terraindata_2::TerrainData_2,
        turn: i32,
        phase: crate::app::force::Force_Type,
        create: bool,
    ) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = "RemoveImpl", args = 2)]
    pub fn remove_impl(self, x: i32, z: i32) -> bool;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl_2(self, i: i32) -> bool;

    #[method(name = "ClearImpl", args = 0)]
    pub fn clear_impl(self) -> ();

    #[method(name = "CreateEffectImpl", args = 0)]
    pub fn create_effect_impl(self) -> ();

    #[method(name = "UpdateEffectImpl", args = 0)]
    pub fn update_effect_impl(self) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(x: i32, z: i32) -> i32;

    #[method(name = "GetCount", args = 0)]
    pub fn get_count() -> i32;

    #[method(name = "GetTerrain", args = 1)]
    pub fn get_terrain(i: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetTerrain", args = 2)]
    pub fn get_terrain_2(x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetX", args = 1)]
    pub fn get_x(i: i32) -> i32;

    #[method(name = "GetZ", args = 1)]
    pub fn get_z(i: i32) -> i32;

    #[method(name = "GetData", args = 1)]
    pub fn get_data(i: i32) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = "GetData", args = 2)]
    pub fn get_data_2(x: i32, z: i32) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = "GetMoveCost", args = 2)]
    pub fn get_move_cost(x: i32, z: i32) -> i32;

    #[method(name = "GetFlyCost", args = 2)]
    pub fn get_fly_cost(x: i32, z: i32) -> i32;

    #[method(name = "Set", args = 5)]
    pub fn set(x: i32, z: i32, index: i32, turn: i32, phase: crate::app::force::Force_Type)
        -> bool;

    #[method(name = "Set", args = 5)]
    pub fn set_2(
        x: i32,
        z: i32,
        tid: ::unity2::Il2CppString,
        turn: i32,
        phase: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "Set", args = 5)]
    pub fn set_3(
        x: i32,
        z: i32,
        terrain: crate::app::terraindata_2::TerrainData_2,
        turn: i32,
        phase: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "Remove", args = 2)]
    pub fn remove(x: i32, z: i32) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove_2(i: i32) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "CreateEffect", args = 0)]
    pub fn create_effect() -> ();

    #[method(name = "UpdateEffect", args = 0)]
    pub fn update_effect() -> ();

    #[method(name = "CanCreation", args = 4)]
    pub fn can_creation(
        attacker: crate::app::unit::Unit,
        x: i32,
        z: i32,
        tid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "CanCreation", args = 4)]
    pub fn can_creation_2(attacker: crate::app::unit::Unit, x: i32, z: i32, index: i32) -> bool;

    #[method(name = "CanCreation", args = 4)]
    pub fn can_creation_3(
        attacker: crate::app::unit::Unit,
        x: i32,
        z: i32,
        data: crate::app::terraindata_2::TerrainData_2,
    ) -> bool;

    #[method(name = "CanRemove", args = 3)]
    pub fn can_remove(attacker: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "FindBreakable", args = 2)]
    pub fn find_breakable(x: i32, z: i32) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapoverlap")]
impl MapOverlap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapOverlap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapOverlapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapoverlap/MapOverlap_List.md")))]
#[::unity2::class(namespace = "App", name = "MapOverlap.List")]
#[parent(crate::system::object::Object)]
pub struct MapOverlap_List {
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::app::mapoverlap::MapOverlap_Data,
    >,
}

#[cfg(feature = "app-mapoverlap")]
#[::unity2::methods]
impl MapOverlap_List {
    #[method(name = "IndexOf", args = 2)]
    pub fn index_of(self, x: i32, z: i32) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, data: crate::app::mapoverlap::MapOverlap_Data) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "CreateEffect", args = 0)]
    pub fn create_effect(self) -> ();

    #[method(name = "UpdateEffect", args = 0)]
    pub fn update_effect(self) -> ();

    #[method(name = "DeleteEffect", args = 0)]
    pub fn delete_effect(self) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::mapoverlap::MapOverlap_Data;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapoverlap")]
impl MapOverlap_List {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapOverlap_List),
                ::core::stringify!(new),
            )
        });
        <Self as IMapOverlap_ListMethods>::ctor(this);
        this
    }
}
