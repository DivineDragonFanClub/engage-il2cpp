
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::gridlayout::GridLayout;
use crate::unity_engine::gridlayout::IGridLayout;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/tilemaps/tilemap/Tilemap.md")))]
#[::unity2::class(namespace = "UnityEngine.Tilemaps", name = "Tilemap")]
#[parent(crate::unity_engine::gridlayout::GridLayout)]
pub struct Tilemap {}

#[cfg(feature = "unity_engine-tilemaps-tilemap")]
#[::unity2::methods]
impl Tilemap {
    #[method(name = "RefreshTile", args = 1)]
    pub fn refresh_tile(self, position: crate::unity_engine::vector3int::Vector3Int) -> ();

    #[method(name = "RefreshTile_Injected", args = 1)]
    pub fn refresh_tile_injected(self, position: crate::unity_engine::vector3int::Vector3Int)
        -> ();
}
