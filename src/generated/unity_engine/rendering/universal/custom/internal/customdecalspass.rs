
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/internal/customdecalspass/CustomDecalsPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom.Internal",
    name = "CustomDecalsPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CustomDecalsPass {
    #[static_field]
    #[rename(name = "m_ProfilerTag")]
    pub m_profiler_tag: ::unity2::Il2CppString,
    #[rename(name = "m_ProfilingSampler")]
    pub m_profiling_sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[rename(name = "m_DecalShadowHandle")]
    pub m_decal_shadow_handle:
        crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[rename(name = "m_DepthHandle")]
    pub m_depth_handle:
        crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[rename(name = "m_FilteringSettings")]
    pub m_filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    #[rename(name = "m_DecalShaderTagId")]
    pub m_decal_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_ScaleBiasId")]
    pub m_scale_bias_id: i32,
    #[rename(name = "m_CopyDepthMaterial")]
    pub m_copy_depth_material: crate::unity_engine::material::Material,
    #[rename(name = "m_IsOcclusionCasterEnabled")]
    pub m_is_occlusion_caster_enabled: bool,
    #[rename(name = "m_OcclusionStrength")]
    pub m_occlusion_strength: f32,
    #[rename(name = "m_OcclusionBlackToAlbedo")]
    pub m_occlusion_black_to_albedo: f32,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customdecalspass")]
#[::unity2::methods]
impl CustomDecalsPass {
    #[method(name = "get_preColorAttachment", args = 0)]
    pub fn get_pre_color_attachment(
        self,
    ) -> crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle;

    #[method(name = "set_preColorAttachment", args = 1)]
    pub fn set_pre_color_attachment(
        self,
        value: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> ();

    #[method(name = "get_preDepthAttachment", args = 0)]
    pub fn get_pre_depth_attachment(
        self,
    ) -> crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle;

    #[method(name = "set_preDepthAttachment", args = 1)]
    pub fn set_pre_depth_attachment(
        self,
        value: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> ();

    #[method(name = "get_srcDepth", args = 0)]
    pub fn get_src_depth(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "set_srcDepth", args = 1)]
    pub fn set_src_depth(
        self,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        layer_mask: crate::unity_engine::layermask::LayerMask,
        copy_depth_material: crate::unity_engine::material::Material,
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

    #[method(name = "SetOcclusionParams", args = 1)]
    pub fn set_occlusion_params(
        self,
        p : crate :: unity_engine :: rendering :: universal :: custom :: decalocclusionparams :: DecalOcclusionParams,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customdecalspass")]
impl CustomDecalsPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        layer_mask: crate::unity_engine::layermask::LayerMask,
        copy_depth_material: crate::unity_engine::material::Material,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomDecalsPass),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomDecalsPassMethods>::ctor(this, evt, layer_mask, copy_depth_material);
        this
    }
}
