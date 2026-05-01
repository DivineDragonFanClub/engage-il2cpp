
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/scriptablerendercontext/ScriptableRenderContext.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ScriptableRenderContext {
    pub m_ptr: ::unity2::IntPtr,
}

impl ::unity2::ClassIdentity for ScriptableRenderContext {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ScriptableRenderContext";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptableRenderContext {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-scriptablerendercontext")]
#[::unity2::methods(value)]
impl ScriptableRenderContext {
    #[method(name = "Internal_Cull", args = 3)]
    pub fn internal_cull(
        parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        render_loop : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        results: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "InitializeSortSettings", args = 2)]
    pub fn initialize_sort_settings(
        camera: crate::unity_engine::camera::Camera,
        sorting_settings: crate::unity_engine::rendering::sortingsettings::SortingSettings,
    ) -> ();

    #[method(name = "Submit_Internal", args = 0)]
    pub fn submit_internal(self) -> ();

    #[method(name = "GetNumberOfCameras_Internal", args = 0)]
    pub fn get_number_of_cameras_internal(self) -> i32;

    #[method(name = "GetCamera_Internal", args = 1)]
    pub fn get_camera_internal(self, index: i32) -> crate::unity_engine::camera::Camera;

    #[method(name = "DrawRenderers_Internal", args = 8)]
    pub fn draw_renderers_internal(
        self,
        cull_results: ::unity2::IntPtr,
        drawing_settings: crate::unity_engine::rendering::drawingsettings::DrawingSettings,
        filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
        tag_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        is_pass_tag_name: bool,
        tag_values: ::unity2::IntPtr,
        state_blocks: ::unity2::IntPtr,
        state_count: i32,
    ) -> ();

    #[method(name = "DrawShadows_Internal", args = 1)]
    pub fn draw_shadows_internal(self, shadow_drawing_settings: ::unity2::IntPtr) -> ();

    #[method(name = "ExecuteCommandBuffer_Internal", args = 1)]
    pub fn execute_command_buffer_internal(
        self,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "ExecuteCommandBufferAsync_Internal", args = 2)]
    pub fn execute_command_buffer_async_internal(
        self,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "SetupCameraProperties_Internal", args = 3)]
    pub fn setup_camera_properties_internal(
        self,
        camera: crate::unity_engine::camera::Camera,
        stereo_setup: bool,
        eye: i32,
    ) -> ();

    #[method(name = "DrawSkybox_Internal", args = 1)]
    pub fn draw_skybox_internal(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "InvokeOnRenderObjectCallback_Internal", args = 0)]
    pub fn invoke_on_render_object_callback_internal(self) -> ();

    #[method(name = "DrawWireOverlay_Impl", args = 1)]
    pub fn draw_wire_overlay_impl(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Submit", args = 0)]
    pub fn submit(self) -> ();

    #[method(name = "GetNumberOfCameras", args = 0)]
    pub fn get_number_of_cameras(self) -> i32;

    #[method(name = "GetCamera", args = 1)]
    pub fn get_camera(self, index: i32) -> crate::unity_engine::camera::Camera;

    #[method(name = "DrawRenderers", args = 3)]
    pub fn draw_renderers(
        self,
        culling_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        drawing_settings: crate::unity_engine::rendering::drawingsettings::DrawingSettings,
        filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    ) -> ();

    #[method(name = "DrawRenderers", args = 4)]
    pub fn draw_renderers_2(
        self,
        culling_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        drawing_settings: crate::unity_engine::rendering::drawingsettings::DrawingSettings,
        filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
        state_block: crate::unity_engine::rendering::renderstateblock::RenderStateBlock,
    ) -> ();

    #[method(name = "DrawShadows", args = 1)]
    pub fn draw_shadows(
        self,
        settings: crate::unity_engine::rendering::shadowdrawingsettings::ShadowDrawingSettings,
    ) -> ();

    #[method(name = "ExecuteCommandBuffer", args = 1)]
    pub fn execute_command_buffer(
        self,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "ExecuteCommandBufferAsync", args = 2)]
    pub fn execute_command_buffer_async(
        self,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "SetupCameraProperties", args = 2)]
    pub fn setup_camera_properties(
        self,
        camera: crate::unity_engine::camera::Camera,
        stereo_setup: bool,
    ) -> ();

    #[method(name = "SetupCameraProperties", args = 3)]
    pub fn setup_camera_properties_2(
        self,
        camera: crate::unity_engine::camera::Camera,
        stereo_setup: bool,
        eye: i32,
    ) -> ();

    #[method(name = "DrawSkybox", args = 1)]
    pub fn draw_skybox(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "InvokeOnRenderObjectCallback", args = 0)]
    pub fn invoke_on_render_object_callback(self) -> ();

    #[method(name = "DrawWireOverlay", args = 1)]
    pub fn draw_wire_overlay(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "Cull", args = 1)]
    pub fn cull(
        self,
        parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> crate::unity_engine::rendering::cullingresults::CullingResults;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "Internal_Cull_Injected", args = 3)]
    pub fn internal_cull_injected(
        parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        render_loop : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        results: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "Submit_Internal_Injected", args = 1)]
    pub fn submit_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
    ) -> ();

    #[method(name = "GetNumberOfCameras_Internal_Injected", args = 1)]
    pub fn get_number_of_cameras_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
    ) -> i32;

    #[method(name = "GetCamera_Internal_Injected", args = 2)]
    pub fn get_camera_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        index: i32,
    ) -> crate::unity_engine::camera::Camera;

    #[method(name = "DrawRenderers_Internal_Injected", args = 9)]
    pub fn draw_renderers_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        cull_results: ::unity2::IntPtr,
        drawing_settings: crate::unity_engine::rendering::drawingsettings::DrawingSettings,
        filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
        tag_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        is_pass_tag_name: bool,
        tag_values: ::unity2::IntPtr,
        state_blocks: ::unity2::IntPtr,
        state_count: i32,
    ) -> ();

    #[method(name = "DrawShadows_Internal_Injected", args = 2)]
    pub fn draw_shadows_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        shadow_drawing_settings: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "ExecuteCommandBuffer_Internal_Injected", args = 2)]
    pub fn execute_command_buffer_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "ExecuteCommandBufferAsync_Internal_Injected", args = 3)]
    pub fn execute_command_buffer_async_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        command_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "SetupCameraProperties_Internal_Injected", args = 4)]
    pub fn setup_camera_properties_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
        stereo_setup: bool,
        eye: i32,
    ) -> ();

    #[method(name = "DrawSkybox_Internal_Injected", args = 2)]
    pub fn draw_skybox_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "InvokeOnRenderObjectCallback_Internal_Injected", args = 1)]
    pub fn invoke_on_render_object_callback_internal_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
    ) -> ();

    #[method(name = "DrawWireOverlay_Impl_Injected", args = 2)]
    pub fn draw_wire_overlay_impl_injected(
        unity_self : crate :: unity_engine :: rendering :: scriptablerendercontext :: ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();
}
