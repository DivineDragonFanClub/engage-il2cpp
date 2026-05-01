
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/depthonlypass/DepthOnlyPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "DepthOnlyPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct DepthOnlyPass {
    #[rename(name = "kDepthBufferBits")]
    pub k_depth_buffer_bits: i32,
    #[rename(name = "m_FilteringSettings")]
    pub m_filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    #[rename(name = "m_ShaderTagId")]
    pub m_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_ShaderTagIdList")]
    pub m_shader_tag_id_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::shadertagid::ShaderTagId,
    >,
}

#[cfg(feature = "unity_engine-rendering-universal-internal-depthonlypass")]
#[::unity2::methods]
impl DepthOnlyPass {
    #[method(name = "get_depthAttachmentHandle", args = 0)]
    pub fn get_depth_attachment_handle(
        self,
    ) -> crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle;

    #[method(name = "set_depthAttachmentHandle", args = 1)]
    pub fn set_depth_attachment_handle(
        self,
        value: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> ();

    #[method(name = "get_descriptor", args = 0)]
    pub fn get_descriptor(
        self,
    ) -> crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor;

    #[method(name = "set_descriptor", args = 1)]
    pub fn set_descriptor(
        self,
        value: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        render_queue_range: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(
        self,
        base_descriptor: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
        depth_attachment_handle : crate :: unity_engine :: rendering :: universal :: rendertargethandle :: RenderTargetHandle,
    ) -> ();

    #[method(name = "OnCameraSetup", args = 2)]
    pub fn on_camera_setup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "OnCameraCleanup", args = 1)]
    pub fn on_camera_cleanup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-internal-depthonlypass")]
impl DepthOnlyPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        render_queue_range: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DepthOnlyPass),
                ::core::stringify!(new),
            )
        });
        <Self as IDepthOnlyPassMethods>::ctor(this, evt, render_queue_range, layer_mask);
        this
    }
}
