
use crate::app::grassmanager::GrassManager;
use crate::app::grassmanager::IGrassManager;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/grassmanagerwithoutterrain/GrassManagerWithoutTerrain.md")))]
#[::unity2::class(namespace = "App", name = "GrassManagerWithoutTerrain")]
#[parent(crate::app::grassmanager::GrassManager)]
pub struct GrassManagerWithoutTerrain {}

#[cfg(feature = "app-grassmanagerwithoutterrain")]
#[::unity2::methods]
impl GrassManagerWithoutTerrain {
    #[method(name = "get_HasTerrain", args = 0)]
    pub fn get_has_terrain(self) -> bool;

    #[method(name = "FindGrassMeshParentTransform", args = 2)]
    pub fn find_grass_mesh_parent_transform(
        self,
        parent: crate::unity_engine::transform::Transform,
        mesh_index: i32,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "CalcReferenceTransformNum", args = 0)]
    pub fn calc_reference_transform_num(self) -> i32;

    #[method(name = "GetFieldSizeOffset", args = 2)]
    pub fn get_field_size_offset(
        self,
        size: crate::unity_engine::vector3::Vector3,
        offset: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "FindGroundTexture", args = 1)]
    pub fn find_ground_texture(self, mesh_index: i32) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-grassmanagerwithoutterrain")]
impl GrassManagerWithoutTerrain {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GrassManagerWithoutTerrain),
                ::core::stringify!(new),
            )
        });
        <Self as IGrassManagerWithoutTerrainMethods>::ctor(this);
        this
    }
}
