
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terrainpaintdata/TerrainPaintData.md")))]
#[::unity2::class(namespace = "App", name = "TerrainPaintData")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct TerrainPaintData {}

#[cfg(feature = "app-terrainpaintdata")]
#[::unity2::methods]
impl TerrainPaintData {
    #[method(name = "set__TerrainSize", args = 1)]
    pub fn set_terrain_size(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get__TerrainSize", args = 0)]
    pub fn get_terrain_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set__SplitCount", args = 1)]
    pub fn set_split_count(self, value: i32) -> ();

    #[method(name = "get__SplitCount", args = 0)]
    pub fn get_split_count(self) -> i32;

    #[method(name = "set__Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get__Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set__Index", args = 1)]
    pub fn set_index(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "get__Index", args = 0)]
    pub fn get_index(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-terrainpaintdata")]
impl TerrainPaintData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainPaintData),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainPaintDataMethods>::ctor(this);
        this
    }
}
