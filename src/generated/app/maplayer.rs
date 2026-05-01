
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maplayer/MapLayer_Data.md")))]
#[::unity2::class(namespace = "App", name = "MapLayer.Data")]
#[parent(crate::system::object::Object)]
pub struct MapLayer_Data {}

#[cfg(feature = "app-maplayer")]
#[::unity2::methods]
impl MapLayer_Data {
    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_W", args = 0)]
    pub fn get_w(self) -> i32;

    #[method(name = "set_W", args = 1)]
    pub fn set_w(self, value: i32) -> ();

    #[method(name = "get_H", args = 0)]
    pub fn get_h(self) -> i32;

    #[method(name = "set_H", args = 1)]
    pub fn set_h(self, value: i32) -> ();

    #[method(name = "get_Group", args = 0)]
    pub fn get_group(self) -> i32;

    #[method(name = "set_Group", args = 1)]
    pub fn set_group(self, value: i32) -> ();

    #[method(name = "get_Tid", args = 0)]
    pub fn get_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tid", args = 1)]
    pub fn set_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_Enable", args = 0)]
    pub fn get_enable(self) -> bool;

    #[method(name = "set_Enable", args = 1)]
    pub fn set_enable(self, value: bool) -> ();

    #[method(name = "IsOutside", args = 2)]
    pub fn is_outside(self, x: i32, z: i32) -> bool;

    #[method(name = "IsInside", args = 2)]
    pub fn is_inside(self, x: i32, z: i32) -> bool;

    #[method(name = "IsConnect", args = 1)]
    pub fn is_connect(self, data: crate::app::maplayer::MapLayer_Data) -> bool;

    #[method(name = "GetTerrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "IsRoof", args = 0)]
    pub fn is_roof(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maplayer")]
impl MapLayer_Data {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapLayer_Data),
                ::core::stringify!(new),
            )
        });
        <Self as IMapLayer_DataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maplayer/MapLayer.md")))]
#[::unity2::class(namespace = "App", name = "MapLayer")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: maplayer :: MapLayer >)]
pub struct MapLayer {
    #[rename(name = "m_List")]
    pub m_list: crate::app::maplayer::MapLayer_List,
}

#[cfg(feature = "app-maplayer")]
#[::unity2::methods]
impl MapLayer {
    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::app::maplayer::MapLayer_Data;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, data: crate::app::maplayer::MapLayer_Data) -> ();

    #[method(name = "Add", args = 7)]
    pub fn add_2(
        self,
        x: i32,
        z: i32,
        w: i32,
        h: i32,
        group: i32,
        tid: ::unity2::Il2CppString,
        enable: bool,
    ) -> ();

    #[method(name = "SetEnable", args = 3)]
    pub fn set_enable(self, x: i32, z: i32, enable: bool) -> ();

    #[method(name = "SetEnable", args = 2)]
    pub fn set_enable_2(self, group: i32, enable: bool) -> ();

    #[method(name = "Find", args = 2)]
    pub fn find(self, x: i32, z: i32) -> crate::app::maplayer::MapLayer_Data;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maplayer")]
impl MapLayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapLayer),
                ::core::stringify!(new),
            )
        });
        <Self as IMapLayerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maplayer/MapLayer_List.md")))]
#[::unity2::class(namespace = "App", name = "MapLayer.List")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: maplayer :: MapLayer_Data >)]
pub struct MapLayer_List {}

#[cfg(feature = "app-maplayer")]
#[::unity2::methods]
impl MapLayer_List {
    #[method(name = "Exists", args = 1)]
    pub fn exists(self, data: crate::app::maplayer::MapLayer_Data) -> bool;

    #[method(name = "IsConnect", args = 1)]
    pub fn is_connect(self, data: crate::app::maplayer::MapLayer_Data) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maplayer")]
impl MapLayer_List {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapLayer_List),
                ::core::stringify!(new),
            )
        });
        <Self as IMapLayer_ListMethods>::ctor(this);
        this
    }
}
