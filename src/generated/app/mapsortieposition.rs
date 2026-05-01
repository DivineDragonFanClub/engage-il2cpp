
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsortieposition/MapSortiePosition_Data.md")))]
#[::unity2::class(namespace = "App", name = "MapSortiePosition.Data")]
#[parent(crate::system::object::Object)]
pub struct MapSortiePosition_Data {}

#[cfg(feature = "app-mapsortieposition")]
#[::unity2::methods]
impl MapSortiePosition_Data {
    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Angle", args = 0)]
    pub fn get_angle(self) -> f32;

    #[method(name = "set_Angle", args = 1)]
    pub fn set_angle(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsortieposition")]
impl MapSortiePosition_Data {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSortiePosition_Data),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSortiePosition_DataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsortieposition/MapSortiePosition.md")))]
#[::unity2::class(namespace = "App", name = "MapSortiePosition")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapsortieposition :: MapSortiePosition >)]
pub struct MapSortiePosition {
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<crate::app::mapsortieposition::MapSortiePosition_Data>,
    #[rename(name = "m_DataCount")]
    pub m_data_count: i32,
}

#[cfg(feature = "app-mapsortieposition")]
#[::unity2::methods]
impl MapSortiePosition {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add(self, x: i32, z: i32, angle: f32) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, index: i32) -> crate::app::mapsortieposition::MapSortiePosition_Data;

    #[method(name = "GetIndex", args = 2)]
    pub fn get_index(self, x: i32, z: i32) -> i32;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-mapsortieposition")]
impl MapSortiePosition {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSortiePosition),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSortiePositionMethods>::ctor(this);
        this
    }
}
