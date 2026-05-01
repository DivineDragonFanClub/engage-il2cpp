
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/scriptablerenderpass/ScriptableRenderPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "ScriptableRenderPass"
)]
#[parent(crate::system::object::Object)]
pub struct ScriptableRenderPass {
# [rename (name = "m_ColorAttachments")] pub m_color_attachments : :: unity2 :: Array < crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier > ,
# [rename (name = "m_DepthAttachment")] pub m_depth_attachment : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier ,
# [rename (name = "m_Input")] pub m_input : crate :: unity_engine :: rendering :: universal :: scriptablerenderpassinput :: ScriptableRenderPassInput ,
# [rename (name = "m_ClearFlag")] pub m_clear_flag : crate :: unity_engine :: rendering :: clearflag :: ClearFlag ,
# [rename (name = "m_ClearColor")] pub m_clear_color : crate :: unity_engine :: color :: Color ,
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerenderpass")]
#[::unity2::methods]
impl ScriptableRenderPass {
    #[method(name = "FrameCleanup", args = 1)]
    pub fn frame_cleanup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "get_renderPassEvent", args = 0)]
    pub fn get_render_pass_event(
        self,
    ) -> crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent;

    #[method(name = "set_renderPassEvent", args = 1)]
    pub fn set_render_pass_event(
        self,
        value: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> ();

    #[method(name = "get_colorAttachments", args = 0)]
    pub fn get_color_attachments(
        self,
    ) -> ::unity2::Array<
        crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    >;

    #[method(name = "get_colorAttachment", args = 0)]
    pub fn get_color_attachment(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "get_depthAttachment", args = 0)]
    pub fn get_depth_attachment(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "get_input", args = 0)]
    pub fn get_input (self ,) -> crate :: unity_engine :: rendering :: universal :: scriptablerenderpassinput :: ScriptableRenderPassInput ;

    #[method(name = "get_clearFlag", args = 0)]
    pub fn get_clear_flag(self) -> crate::unity_engine::rendering::clearflag::ClearFlag;

    #[method(name = "get_clearColor", args = 0)]
    pub fn get_clear_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "get_profilingSampler", args = 0)]
    pub fn get_profiling_sampler(
        self,
    ) -> crate::unity_engine::rendering::profilingsampler::ProfilingSampler;

    #[method(name = "set_profilingSampler", args = 1)]
    pub fn set_profiling_sampler(
        self,
        value: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    ) -> ();

    #[method(name = "get_overrideCameraTarget", args = 0)]
    pub fn get_override_camera_target(self) -> bool;

    #[method(name = "set_overrideCameraTarget", args = 1)]
    pub fn set_override_camera_target(self, value: bool) -> ();

    #[method(name = "get_isBlitRenderPass", args = 0)]
    pub fn get_is_blit_render_pass(self) -> bool;

    #[method(name = "set_isBlitRenderPass", args = 1)]
    pub fn set_is_blit_render_pass(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ConfigureInput", args = 1)]
    pub fn configure_input(
        self,
        pass_input : crate :: unity_engine :: rendering :: universal :: scriptablerenderpassinput :: ScriptableRenderPassInput,
    ) -> ();

    #[method(name = "ConfigureTarget", args = 2)]
    pub fn configure_target(
        self,
        color_attachment : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_attachment : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "ConfigureTarget", args = 2)]
    pub fn configure_target_2(
        self,
        color_attachments: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
        depth_attachment : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "ConfigureTarget", args = 1)]
    pub fn configure_target_3(
        self,
        color_attachment : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "ConfigureTarget", args = 1)]
    pub fn configure_target_4(
        self,
        color_attachments: ::unity2::Array<
            crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        >,
    ) -> ();

    #[method(name = "ConfigureClear", args = 2)]
    pub fn configure_clear(
        self,
        clear_flag: crate::unity_engine::rendering::clearflag::ClearFlag,
        clear_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "OnCameraSetup", args = 2)]
    pub fn on_camera_setup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "Configure", args = 2)]
    pub fn configure(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera_texture_descriptor : crate :: unity_engine :: rendertexturedescriptor :: RenderTextureDescriptor,
    ) -> ();

    #[method(name = "OnCameraCleanup", args = 1)]
    pub fn on_camera_cleanup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "OnFinishCameraStackRendering", args = 1)]
    pub fn on_finish_camera_stack_rendering(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "Blit", args = 5)]
    pub fn blit(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        destination: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        material: crate::unity_engine::material::Material,
        pass_index: i32,
    ) -> ();

    #[method(name = "CreateDrawingSettings", args = 3)]
    pub fn create_drawing_settings(
        self,
        shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
        sorting_criteria: crate::unity_engine::rendering::sortingcriteria::SortingCriteria,
    ) -> crate::unity_engine::rendering::drawingsettings::DrawingSettings;

    #[method(name = "CreateDrawingSettings", args = 3)]
    pub fn create_drawing_settings_2(
        self,
        shader_tag_id_list: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::shadertagid::ShaderTagId,
        >,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
        sorting_criteria: crate::unity_engine::rendering::sortingcriteria::SortingCriteria,
    ) -> crate::unity_engine::rendering::drawingsettings::DrawingSettings;

    #[method(name = "op_LessThan", args = 2)]
    pub fn op_less_than(
        lhs: crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass,
        rhs: crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass,
    ) -> bool;

    #[method(name = "op_GreaterThan", args = 2)]
    pub fn op_greater_than(
        lhs: crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass,
        rhs: crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass,
    ) -> bool;
}

#[cfg(feature = "unity_engine-rendering-universal-scriptablerenderpass")]
impl ScriptableRenderPass {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptableRenderPass),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptableRenderPassMethods>::ctor(this);
        this
    }
}
