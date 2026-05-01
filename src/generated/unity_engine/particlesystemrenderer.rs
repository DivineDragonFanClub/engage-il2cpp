
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::renderer::IRenderer;
use crate::unity_engine::renderer::Renderer;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystemrenderer/ParticleSystemRenderer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ParticleSystemRenderer")]
#[parent(crate::unity_engine::renderer::Renderer)]
pub struct ParticleSystemRenderer {}

#[cfg(feature = "unity_engine-particlesystemrenderer")]
#[::unity2::methods]
impl ParticleSystemRenderer {
    #[method(name = "EnableVertexStreams", args = 1)]
    pub fn enable_vertex_streams(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
    ) -> ();

    #[method(name = "DisableVertexStreams", args = 1)]
    pub fn disable_vertex_streams(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
    ) -> ();

    #[method(name = "AreVertexStreamsEnabled", args = 1)]
    pub fn are_vertex_streams_enabled(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
    ) -> bool;

    #[method(name = "GetEnabledVertexStreams", args = 1)]
    pub fn get_enabled_vertex_streams(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
    ) -> crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams;

    #[method(name = "Internal_SetVertexStreams", args = 2)]
    pub fn internal_set_vertex_streams(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
        enabled: bool,
    ) -> ();

    #[method(name = "Internal_GetEnabledVertexStreams", args = 1)]
    pub fn internal_get_enabled_vertex_streams(
        self,
        streams: crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams,
    ) -> crate::unity_engine::particlesystemvertexstreams::ParticleSystemVertexStreams;

    #[method(name = "get_alignment", args = 0)]
    pub fn get_alignment(
        self,
    ) -> crate::unity_engine::particlesystemrenderspace::ParticleSystemRenderSpace;

    #[method(name = "set_alignment", args = 1)]
    pub fn set_alignment(
        self,
        value: crate::unity_engine::particlesystemrenderspace::ParticleSystemRenderSpace,
    ) -> ();

    #[method(name = "get_renderMode", args = 0)]
    pub fn get_render_mode(
        self,
    ) -> crate::unity_engine::particlesystemrendermode::ParticleSystemRenderMode;

    #[method(name = "set_renderMode", args = 1)]
    pub fn set_render_mode(
        self,
        value: crate::unity_engine::particlesystemrendermode::ParticleSystemRenderMode,
    ) -> ();

    #[method(name = "get_sortMode", args = 0)]
    pub fn get_sort_mode(
        self,
    ) -> crate::unity_engine::particlesystemsortmode::ParticleSystemSortMode;

    #[method(name = "set_sortMode", args = 1)]
    pub fn set_sort_mode(
        self,
        value: crate::unity_engine::particlesystemsortmode::ParticleSystemSortMode,
    ) -> ();

    #[method(name = "get_lengthScale", args = 0)]
    pub fn get_length_scale(self) -> f32;

    #[method(name = "set_lengthScale", args = 1)]
    pub fn set_length_scale(self, value: f32) -> ();

    #[method(name = "get_velocityScale", args = 0)]
    pub fn get_velocity_scale(self) -> f32;

    #[method(name = "set_velocityScale", args = 1)]
    pub fn set_velocity_scale(self, value: f32) -> ();

    #[method(name = "get_cameraVelocityScale", args = 0)]
    pub fn get_camera_velocity_scale(self) -> f32;

    #[method(name = "set_cameraVelocityScale", args = 1)]
    pub fn set_camera_velocity_scale(self, value: f32) -> ();

    #[method(name = "get_normalDirection", args = 0)]
    pub fn get_normal_direction(self) -> f32;

    #[method(name = "set_normalDirection", args = 1)]
    pub fn set_normal_direction(self, value: f32) -> ();

    #[method(name = "get_shadowBias", args = 0)]
    pub fn get_shadow_bias(self) -> f32;

    #[method(name = "set_shadowBias", args = 1)]
    pub fn set_shadow_bias(self, value: f32) -> ();

    #[method(name = "get_sortingFudge", args = 0)]
    pub fn get_sorting_fudge(self) -> f32;

    #[method(name = "set_sortingFudge", args = 1)]
    pub fn set_sorting_fudge(self, value: f32) -> ();

    #[method(name = "get_minParticleSize", args = 0)]
    pub fn get_min_particle_size(self) -> f32;

    #[method(name = "set_minParticleSize", args = 1)]
    pub fn set_min_particle_size(self, value: f32) -> ();

    #[method(name = "get_maxParticleSize", args = 0)]
    pub fn get_max_particle_size(self) -> f32;

    #[method(name = "set_maxParticleSize", args = 1)]
    pub fn set_max_particle_size(self, value: f32) -> ();

    #[method(name = "get_pivot", args = 0)]
    pub fn get_pivot(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_pivot", args = 1)]
    pub fn set_pivot(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_flip", args = 0)]
    pub fn get_flip(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_flip", args = 1)]
    pub fn set_flip(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_maskInteraction", args = 0)]
    pub fn get_mask_interaction(
        self,
    ) -> crate::unity_engine::spritemaskinteraction::SpriteMaskInteraction;

    #[method(name = "set_maskInteraction", args = 1)]
    pub fn set_mask_interaction(
        self,
        value: crate::unity_engine::spritemaskinteraction::SpriteMaskInteraction,
    ) -> ();

    #[method(name = "get_trailMaterial", args = 0)]
    pub fn get_trail_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_trailMaterial", args = 1)]
    pub fn set_trail_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_enableGPUInstancing", args = 0)]
    pub fn get_enable_gpu_instancing(self) -> bool;

    #[method(name = "set_enableGPUInstancing", args = 1)]
    pub fn set_enable_gpu_instancing(self, value: bool) -> ();

    #[method(name = "get_allowRoll", args = 0)]
    pub fn get_allow_roll(self) -> bool;

    #[method(name = "set_allowRoll", args = 1)]
    pub fn set_allow_roll(self, value: bool) -> ();

    #[method(name = "get_freeformStretching", args = 0)]
    pub fn get_freeform_stretching(self) -> bool;

    #[method(name = "set_freeformStretching", args = 1)]
    pub fn set_freeform_stretching(self, value: bool) -> ();

    #[method(name = "get_rotateWithStretchDirection", args = 0)]
    pub fn get_rotate_with_stretch_direction(self) -> bool;

    #[method(name = "set_rotateWithStretchDirection", args = 1)]
    pub fn set_rotate_with_stretch_direction(self, value: bool) -> ();

    #[method(name = "get_mesh", args = 0)]
    pub fn get_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "set_mesh", args = 1)]
    pub fn set_mesh(self, value: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "GetMeshes", args = 1)]
    pub fn get_meshes(self, meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>) -> i32;

    #[method(name = "SetMeshes", args = 2)]
    pub fn set_meshes(
        self,
        meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>,
        size: i32,
    ) -> ();

    #[method(name = "SetMeshes", args = 1)]
    pub fn set_meshes_2(self, meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>) -> ();

    #[method(name = "get_meshCount", args = 0)]
    pub fn get_mesh_count(self) -> i32;

    #[method(name = "BakeMesh", args = 2)]
    pub fn bake_mesh(self, mesh: crate::unity_engine::mesh::Mesh, use_transform: bool) -> ();

    #[method(name = "BakeMesh", args = 3)]
    pub fn bake_mesh_2(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        camera: crate::unity_engine::camera::Camera,
        use_transform: bool,
    ) -> ();

    #[method(name = "BakeTrailsMesh", args = 2)]
    pub fn bake_trails_mesh(self, mesh: crate::unity_engine::mesh::Mesh, use_transform: bool)
        -> ();

    #[method(name = "BakeTrailsMesh", args = 3)]
    pub fn bake_trails_mesh_2(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        camera: crate::unity_engine::camera::Camera,
        use_transform: bool,
    ) -> ();

    #[method(name = "get_activeVertexStreamsCount", args = 0)]
    pub fn get_active_vertex_streams_count(self) -> i32;

    #[method(name = "SetActiveVertexStreams", args = 1)]
    pub fn set_active_vertex_streams(
        self,
        streams: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::particlesystemvertexstream::ParticleSystemVertexStream,
        >,
    ) -> ();

    #[method(name = "GetActiveVertexStreams", args = 1)]
    pub fn get_active_vertex_streams(
        self,
        streams: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::particlesystemvertexstream::ParticleSystemVertexStream,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_pivot_Injected", args = 1)]
    pub fn get_pivot_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_pivot_Injected", args = 1)]
    pub fn set_pivot_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_flip_Injected", args = 1)]
    pub fn get_flip_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_flip_Injected", args = 1)]
    pub fn set_flip_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg(feature = "unity_engine-particlesystemrenderer")]
impl ParticleSystemRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ParticleSystemRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as IParticleSystemRendererMethods>::ctor(this);
        this
    }
}
