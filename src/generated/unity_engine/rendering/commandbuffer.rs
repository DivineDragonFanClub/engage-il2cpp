
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/commandbuffer/CommandBuffer.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "CommandBuffer")]
#[parent(crate::system::object::Object)]
pub struct CommandBuffer {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-rendering-commandbuffer")]
#[::unity2::methods]
impl CommandBuffer {
    #[method(name = "Internal_SetSinglePassStereo", args = 1)]
    pub fn internal_set_single_pass_stereo(
        self,
        mode: crate::unity_engine::rendering::singlepassstereomode::SinglePassStereoMode,
    ) -> ();

    #[method(name = "InitBuffer", args = 0)]
    pub fn init_buffer() -> ::unity2::IntPtr;

    #[method(name = "CreateGPUFence_Internal", args = 2)]
    pub fn create_gpu_fence_internal(
        self,
        fence_type: crate::unity_engine::rendering::graphicsfencetype::GraphicsFenceType,
        stage: crate::unity_engine::rendering::synchronisationstageflags::SynchronisationStageFlags,
    ) -> ::unity2::IntPtr;

    #[method(name = "WaitOnGPUFence_Internal", args = 2)]
    pub fn wait_on_gpu_fence_internal(
        self,
        fence_ptr: ::unity2::IntPtr,
        stage: crate::unity_engine::rendering::synchronisationstageflags::SynchronisationStageFlags,
    ) -> ();

    #[method(name = "ReleaseBuffer", args = 0)]
    pub fn release_buffer(self) -> ();

    #[method(name = "Internal_SetComputeTextureParam", args = 6)]
    pub fn internal_set_compute_texture_param(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        kernel_index: i32,
        name_id: i32,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mip_level: i32,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "Internal_SetComputeConstantComputeBufferParam", args = 5)]
    pub fn internal_set_compute_constant_compute_buffer_param(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        name_id: i32,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "Internal_DispatchCompute", args = 5)]
    pub fn internal_dispatch_compute(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        kernel_index: i32,
        thread_groups_x: i32,
        thread_groups_y: i32,
        thread_groups_z: i32,
    ) -> ();

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Internal_DrawMesh", args = 6)]
    pub fn internal_draw_mesh(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "Internal_DrawRenderer", args = 4)]
    pub fn internal_draw_renderer(
        self,
        renderer: crate::unity_engine::renderer::Renderer,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
    ) -> ();

    #[method(name = "Internal_DrawProcedural", args = 7)]
    pub fn internal_draw_procedural(
        self,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        shader_pass: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        vertex_count: i32,
        instance_count: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "Internal_DrawOcclusionMesh", args = 1)]
    pub fn internal_draw_occlusion_mesh(
        self,
        normalized_cam_viewport: crate::unity_engine::rectint::RectInt,
    ) -> ();

    #[method(name = "SetViewport", args = 1)]
    pub fn set_viewport(self, pixel_rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "EnableScissorRect", args = 1)]
    pub fn enable_scissor_rect(self, scissor: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "DisableScissorRect", args = 0)]
    pub fn disable_scissor_rect(self) -> ();

    #[method(name = "Blit_Identifier", args = 8)]
    pub fn blit_identifier(
        self,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        dest: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mat: crate::unity_engine::material::Material,
        pass: i32,
        scale: crate::unity_engine::vector2::Vector2,
        offset: crate::unity_engine::vector2::Vector2,
        source_depth_slice: i32,
        dest_depth_slice: i32,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 10)]
    pub fn get_temporary_rt(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
        enable_random_write: bool,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        use_dynamic_scale: bool,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 11)]
    pub fn get_temporary_rt_2(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        enable_random_write: bool,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        use_dynamic_scale: bool,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 10)]
    pub fn get_temporary_rt_3(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        enable_random_write: bool,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 9)]
    pub fn get_temporary_rt_4(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        enable_random_write: bool,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 8)]
    pub fn get_temporary_rt_5(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 7)]
    pub fn get_temporary_rt_6(
        self,
        name_id: i32,
        width: i32,
        height: i32,
        depth_buffer: i32,
        filter: crate::unity_engine::filtermode::FilterMode,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> ();

    #[method(name = "GetTemporaryRTWithDescriptor", args = 3)]
    pub fn get_temporary_rt_with_descriptor(
        self,
        name_id: i32,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
        filter: crate::unity_engine::filtermode::FilterMode,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 3)]
    pub fn get_temporary_rt_7(
        self,
        name_id: i32,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
        filter: crate::unity_engine::filtermode::FilterMode,
    ) -> ();

    #[method(name = "GetTemporaryRT", args = 2)]
    pub fn get_temporary_rt_8(
        self,
        name_id: i32,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "ReleaseTemporaryRT", args = 1)]
    pub fn release_temporary_rt(self, name_id: i32) -> ();

    #[method(name = "ClearRenderTarget", args = 4)]
    pub fn clear_render_target(
        self,
        clear_depth: bool,
        clear_color: bool,
        background_color: crate::unity_engine::color::Color,
        depth: f32,
    ) -> ();

    #[method(name = "ClearRenderTarget", args = 3)]
    pub fn clear_render_target_2(
        self,
        clear_depth: bool,
        clear_color: bool,
        background_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetGlobalFloat", args = 2)]
    pub fn set_global_float(self, name_id: i32, value: f32) -> ();

    #[method(name = "SetGlobalInt", args = 2)]
    pub fn set_global_int(self, name_id: i32, value: i32) -> ();

    #[method(name = "SetGlobalVector", args = 2)]
    pub fn set_global_vector(
        self,
        name_id: i32,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetGlobalColor", args = 2)]
    pub fn set_global_color(self, name_id: i32, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetGlobalMatrix", args = 2)]
    pub fn set_global_matrix(
        self,
        name_id: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "EnableShaderKeyword", args = 1)]
    pub fn enable_shader_keyword(self, keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "DisableShaderKeyword", args = 1)]
    pub fn disable_shader_keyword(self, keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "SetViewProjectionMatrices", args = 2)]
    pub fn set_view_projection_matrices(
        self,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetExecutionFlags", args = 1)]
    pub fn set_execution_flags(
        self,
        flags : crate :: unity_engine :: rendering :: commandbufferexecutionflags :: CommandBufferExecutionFlags,
    ) -> ();

    #[method(name = "ValidateAgainstExecutionFlags", args = 2)]
    pub fn validate_against_execution_flags(
        self,
        required_flags : crate :: unity_engine :: rendering :: commandbufferexecutionflags :: CommandBufferExecutionFlags,
        invalid_flags : crate :: unity_engine :: rendering :: commandbufferexecutionflags :: CommandBufferExecutionFlags,
    ) -> bool;

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array(
        self,
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 2)]
    pub fn set_global_matrix_array(
        self,
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "SetGlobalTexture_Impl", args = 3)]
    pub fn set_global_texture_impl(
        self,
        name_id: i32,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetGlobalBufferInternal", args = 2)]
    pub fn set_global_buffer_internal(
        self,
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "BeginSample", args = 1)]
    pub fn begin_sample(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "EndSample", args = 1)]
    pub fn end_sample(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "BeginSample", args = 1)]
    pub fn begin_sample_2(
        self,
        sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "EndSample", args = 1)]
    pub fn end_sample_2(
        self,
        sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "BeginSample_CustomSampler", args = 1)]
    pub fn begin_sample_custom_sampler(
        self,
        sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "EndSample_CustomSampler", args = 1)]
    pub fn end_sample_custom_sampler(
        self,
        sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "SetGlobalConstantBufferInternal", args = 4)]
    pub fn set_global_constant_buffer_internal(
        self,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        name_id: i32,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetInstanceMultiplier", args = 1)]
    pub fn set_instance_multiplier(self, multiplier: u32) -> ();

    #[method(name = "SetRenderTarget", args = 1)]
    pub fn set_render_target(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 3)]
    pub fn set_render_target_2(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        load_action: crate::unity_engine::rendering::renderbufferloadaction::RenderBufferLoadAction,
        store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_3(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 4)]
    pub fn set_render_target_4(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mip_level: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 2)]
    pub fn set_render_target_5(
        self,
        color: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_6(
        self,
        color: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mip_level: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 6)]
    pub fn set_render_target_7(
        self,
        color: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "SetRenderTarget", args = 5)]
    pub fn set_render_target_8(
        self,
        colors: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mip_level: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetRenderTargetSingle_Internal", args = 5)]
    pub fn set_render_target_single_internal(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "SetRenderTargetColorDepth_Internal", args = 7)]
    pub fn set_render_target_color_depth_internal(
        self,
        color: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        flags: crate::unity_engine::rendering::rendertargetflags::RenderTargetFlags,
    ) -> ();

    #[method(name = "SetRenderTargetMultiSubtarget", args = 9)]
    pub fn set_render_target_multi_subtarget(
        self,
        colors: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_actions: ::unity2::Array<
            crate::unity_engine::rendering::renderbufferloadaction::RenderBufferLoadAction,
        >,
        color_store_actions: ::unity2::Array<
            crate::unity_engine::rendering::renderbufferstoreaction::RenderBufferStoreAction,
        >,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        mip_level: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "SetComputeBufferData", args = 2)]
    pub fn set_compute_buffer_data(
        self,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        data: ::unity2::IlInstance,
    ) -> ();

    #[method(name = "InternalSetComputeBufferData", args = 6)]
    pub fn internal_set_compute_buffer_data(
        self,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        data: ::unity2::IlInstance,
        managed_buffer_start_index: i32,
        graphics_buffer_start_index: i32,
        count: i32,
        elem_size: i32,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateAsyncGraphicsFence", args = 0)]
    pub fn create_async_graphics_fence(
        self,
    ) -> crate::unity_engine::rendering::graphicsfence::GraphicsFence;

    #[method(name = "CreateGraphicsFence", args = 2)]
    pub fn create_graphics_fence(
        self,
        fence_type: crate::unity_engine::rendering::graphicsfencetype::GraphicsFenceType,
        stage: crate::unity_engine::rendering::synchronisationstageflags::SynchronisationStageFlags,
    ) -> crate::unity_engine::rendering::graphicsfence::GraphicsFence;

    #[method(name = "WaitOnAsyncGraphicsFence", args = 1)]
    pub fn wait_on_async_graphics_fence(
        self,
        fence: crate::unity_engine::rendering::graphicsfence::GraphicsFence,
    ) -> ();

    #[method(name = "WaitOnAsyncGraphicsFence", args = 2)]
    pub fn wait_on_async_graphics_fence_2(
        self,
        fence: crate::unity_engine::rendering::graphicsfence::GraphicsFence,
        stage: crate::unity_engine::rendering::synchronisationstage::SynchronisationStage,
    ) -> ();

    #[method(name = "WaitOnAsyncGraphicsFence", args = 2)]
    pub fn wait_on_async_graphics_fence_3(
        self,
        fence: crate::unity_engine::rendering::graphicsfence::GraphicsFence,
        stage: crate::unity_engine::rendering::synchronisationstageflags::SynchronisationStageFlags,
    ) -> ();

    #[method(name = "SetComputeTextureParam", args = 4)]
    pub fn set_compute_texture_param(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        kernel_index: i32,
        name: ::unity2::Il2CppString,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetComputeConstantBufferParam", args = 5)]
    pub fn set_compute_constant_buffer_param(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        name_id: i32,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "DispatchCompute", args = 5)]
    pub fn dispatch_compute(
        self,
        compute_shader: crate::unity_engine::computeshader::ComputeShader,
        kernel_index: i32,
        thread_groups_x: i32,
        thread_groups_y: i32,
        thread_groups_z: i32,
    ) -> ();

    #[method(name = "DrawMesh", args = 6)]
    pub fn draw_mesh(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "DrawMesh", args = 5)]
    pub fn draw_mesh_2(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
    ) -> ();

    #[method(name = "DrawMesh", args = 4)]
    pub fn draw_mesh_3(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
    ) -> ();

    #[method(name = "DrawMesh", args = 3)]
    pub fn draw_mesh_4(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "DrawRenderer", args = 4)]
    pub fn draw_renderer(
        self,
        renderer: crate::unity_engine::renderer::Renderer,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
    ) -> ();

    #[method(name = "DrawRenderer", args = 3)]
    pub fn draw_renderer_2(
        self,
        renderer: crate::unity_engine::renderer::Renderer,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
    ) -> ();

    #[method(name = "DrawRenderer", args = 2)]
    pub fn draw_renderer_3(
        self,
        renderer: crate::unity_engine::renderer::Renderer,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "DrawProcedural", args = 7)]
    pub fn draw_procedural(
        self,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        shader_pass: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        vertex_count: i32,
        instance_count: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "DrawProcedural", args = 6)]
    pub fn draw_procedural_2(
        self,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        shader_pass: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        vertex_count: i32,
        instance_count: i32,
    ) -> ();

    #[method(name = "DrawProcedural", args = 5)]
    pub fn draw_procedural_3(
        self,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        shader_pass: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        vertex_count: i32,
    ) -> ();

    #[method(name = "DrawOcclusionMesh", args = 1)]
    pub fn draw_occlusion_mesh(
        self,
        normalized_cam_viewport: crate::unity_engine::rectint::RectInt,
    ) -> ();

    #[method(name = "Blit", args = 2)]
    pub fn blit(
        self,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        dest: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "Blit", args = 4)]
    pub fn blit_2(
        self,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        dest: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mat: crate::unity_engine::material::Material,
        pass: i32,
    ) -> ();

    #[method(name = "SetGlobalVector", args = 2)]
    pub fn set_global_vector_2(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array_2(
        self,
        property_name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 2)]
    pub fn set_global_texture(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 2)]
    pub fn set_global_texture_2(
        self,
        name_id: i32,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 3)]
    pub fn set_global_texture_3(
        self,
        name_id: i32,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetGlobalBuffer", args = 2)]
    pub fn set_global_buffer(
        self,
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetGlobalConstantBuffer", args = 4)]
    pub fn set_global_constant_buffer(
        self,
        buffer: crate::unity_engine::computebuffer::ComputeBuffer,
        name_id: i32,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetSinglePassStereo", args = 1)]
    pub fn set_single_pass_stereo(
        self,
        mode: crate::unity_engine::rendering::singlepassstereomode::SinglePassStereoMode,
    ) -> ();

    #[method(name = "Internal_DrawMesh_Injected", args = 6)]
    pub fn internal_draw_mesh_injected(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        submesh_index: i32,
        shader_pass: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "Internal_DrawProcedural_Injected", args = 7)]
    pub fn internal_draw_procedural_injected(
        self,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
        material: crate::unity_engine::material::Material,
        shader_pass: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        vertex_count: i32,
        instance_count: i32,
        properties: crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    ) -> ();

    #[method(name = "Internal_DrawOcclusionMesh_Injected", args = 1)]
    pub fn internal_draw_occlusion_mesh_injected(
        self,
        normalized_cam_viewport: crate::unity_engine::rectint::RectInt,
    ) -> ();

    #[method(name = "SetViewport_Injected", args = 1)]
    pub fn set_viewport_injected(self, pixel_rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "EnableScissorRect_Injected", args = 1)]
    pub fn enable_scissor_rect_injected(self, scissor: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "Blit_Identifier_Injected", args = 8)]
    pub fn blit_identifier_injected(
        self,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        dest: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        mat: crate::unity_engine::material::Material,
        pass: i32,
        scale: crate::unity_engine::vector2::Vector2,
        offset: crate::unity_engine::vector2::Vector2,
        source_depth_slice: i32,
        dest_depth_slice: i32,
    ) -> ();

    #[method(name = "GetTemporaryRTWithDescriptor_Injected", args = 3)]
    pub fn get_temporary_rt_with_descriptor_injected(
        self,
        name_id: i32,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
        filter: crate::unity_engine::filtermode::FilterMode,
    ) -> ();

    #[method(name = "ClearRenderTarget_Injected", args = 4)]
    pub fn clear_render_target_injected(
        self,
        clear_depth: bool,
        clear_color: bool,
        background_color: crate::unity_engine::color::Color,
        depth: f32,
    ) -> ();

    #[method(name = "SetGlobalVector_Injected", args = 2)]
    pub fn set_global_vector_injected(
        self,
        name_id: i32,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetGlobalColor_Injected", args = 2)]
    pub fn set_global_color_injected(
        self,
        name_id: i32,
        value: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetGlobalMatrix_Injected", args = 2)]
    pub fn set_global_matrix_injected(
        self,
        name_id: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetViewProjectionMatrices_Injected", args = 2)]
    pub fn set_view_projection_matrices_injected(
        self,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetRenderTargetSingle_Internal_Injected", args = 5)]
    pub fn set_render_target_single_internal_injected(
        self,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
    ) -> ();

    #[method(name = "SetRenderTargetColorDepth_Internal_Injected", args = 7)]
    pub fn set_render_target_color_depth_internal_injected(
        self,
        color: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        color_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        flags: crate::unity_engine::rendering::rendertargetflags::RenderTargetFlags,
    ) -> ();

    #[method(name = "SetRenderTargetMultiSubtarget_Injected", args = 9)]
    pub fn set_render_target_multi_subtarget_injected(
        self,
        colors: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        color_load_actions: ::unity2::Array<
            crate::unity_engine::rendering::renderbufferloadaction::RenderBufferLoadAction,
        >,
        color_store_actions: ::unity2::Array<
            crate::unity_engine::rendering::renderbufferstoreaction::RenderBufferStoreAction,
        >,
        depth_load_action : crate :: unity_engine :: rendering :: renderbufferloadaction :: RenderBufferLoadAction,
        depth_store_action : crate :: unity_engine :: rendering :: renderbufferstoreaction :: RenderBufferStoreAction,
        mip_level: i32,
        cubemap_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-commandbuffer")]
impl CommandBuffer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommandBuffer),
                ::core::stringify!(new),
            )
        });
        <Self as ICommandBufferMethods>::ctor(this);
        this
    }
}
