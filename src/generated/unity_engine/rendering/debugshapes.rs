
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugshapes/DebugShapes.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DebugShapes")]
#[parent(crate::system::object::Object)]
pub struct DebugShapes {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::unity_engine::rendering::debugshapes::DebugShapes,
    #[rename(name = "m_sphereMesh")]
    pub m_sphere_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_boxMesh")]
    pub m_box_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_coneMesh")]
    pub m_cone_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_pyramidMesh")]
    pub m_pyramid_mesh: crate::unity_engine::mesh::Mesh,
}

#[cfg(feature = "unity_engine-rendering-debugshapes")]
#[::unity2::methods]
impl DebugShapes {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::unity_engine::rendering::debugshapes::DebugShapes;

    #[method(name = "BuildSphere", args = 4)]
    pub fn build_sphere(
        self,
        output_mesh: crate::unity_engine::mesh::Mesh,
        radius: f32,
        long_subdiv: u32,
        lat_subdiv: u32,
    ) -> ();

    #[method(name = "BuildBox", args = 4)]
    pub fn build_box(
        self,
        output_mesh: crate::unity_engine::mesh::Mesh,
        length: f32,
        width: f32,
        height: f32,
    ) -> ();

    #[method(name = "BuildCone", args = 5)]
    pub fn build_cone(
        self,
        output_mesh: crate::unity_engine::mesh::Mesh,
        height: f32,
        top_radius: f32,
        bottom_radius: f32,
        nb_sides: i32,
    ) -> ();

    #[method(name = "BuildPyramid", args = 4)]
    pub fn build_pyramid(
        self,
        output_mesh: crate::unity_engine::mesh::Mesh,
        width: f32,
        height: f32,
        depth: f32,
    ) -> ();

    #[method(name = "BuildShapes", args = 0)]
    pub fn build_shapes(self) -> ();

    #[method(name = "RebuildResources", args = 0)]
    pub fn rebuild_resources(self) -> ();

    #[method(name = "RequestSphereMesh", args = 0)]
    pub fn request_sphere_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "RequestBoxMesh", args = 0)]
    pub fn request_box_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "RequestConeMesh", args = 0)]
    pub fn request_cone_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "RequestPyramidMesh", args = 0)]
    pub fn request_pyramid_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-debugshapes")]
impl DebugShapes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugShapes),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugShapesMethods>::ctor(this);
        this
    }
}
