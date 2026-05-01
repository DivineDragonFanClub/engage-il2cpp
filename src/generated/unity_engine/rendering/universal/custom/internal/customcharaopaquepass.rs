
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/internal/customcharaopaquepass/CustomCharaOpaquePass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom.Internal",
    name = "CustomCharaOpaquePass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CustomCharaOpaquePass {
    #[static_field]
    #[rename(name = "m_ProfilerTag")]
    pub m_profiler_tag: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_ProfilerTag2")]
    pub m_profiler_tag2: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_ProfilerTag3")]
    pub m_profiler_tag3: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_ProfilerTag4")]
    pub m_profiler_tag4: ::unity2::Il2CppString,
    #[rename(name = "m_ProfilingSampler")]
    pub m_profiling_sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[rename(name = "m_ProfilingSampler2")]
    pub m_profiling_sampler2: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[rename(name = "m_ProfilingSampler3")]
    pub m_profiling_sampler3: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[rename(name = "m_ProfilingSampler4")]
    pub m_profiling_sampler4: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[static_field]
    #[rename(name = "s_DrawObjectPassDataPropID")]
    pub s_draw_object_pass_data_prop_id: i32,
    #[rename(name = "m_FilteringSettings")]
    pub m_filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    #[rename(name = "m_BaseShaderTagId")]
    pub m_base_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_HairShaderTagId")]
    pub m_hair_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_EyeShaderTagId")]
    pub m_eye_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_BrowShadowShaderTagId")]
    pub m_brow_shadow_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_OpaqueBlendShaderTagId")]
    pub m_opaque_blend_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_OutlineShaderTagId")]
    pub m_outline_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_SilhouetteShaderTagId")]
    pub m_silhouette_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_DepthOnlyShaderTagId")]
    pub m_depth_only_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_OverrideRenderStateBlock")]
    pub m_override_render_state_block:
        crate::unity_engine::rendering::renderstateblock::RenderStateBlock,
    #[rename(name = "m_OverrideDepthState")]
    pub m_override_depth_state: crate::unity_engine::rendering::depthstate::DepthState,
    #[rename(name = "m_CustomViewport")]
    pub m_custom_viewport: bool,
    #[rename(name = "m_CustomViewportRect")]
    pub m_custom_viewport_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_OriginalViewportRect")]
    pub m_original_viewport_rect: crate::unity_engine::rect::Rect,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customcharaopaquepass")]
#[::unity2::methods]
impl CustomCharaOpaquePass {
    #[method(name = "get_statusRenderingFlag", args = 0)]
    pub fn get_status_rendering_flag(self) -> bool;

    #[method(name = "set_statusRenderingFlag", args = 1)]
    pub fn set_status_rendering_flag(self, value: bool) -> ();

    #[method(name = "get_silhouetteFlag", args = 0)]
    pub fn get_silhouette_flag(self) -> bool;

    #[method(name = "set_silhouetteFlag", args = 1)]
    pub fn set_silhouette_flag(self, value: bool) -> ();

    #[method(name = "get_outlineFlag", args = 0)]
    pub fn get_outline_flag(self) -> bool;

    #[method(name = "set_outlineFlag", args = 1)]
    pub fn set_outline_flag(self, value: bool) -> ();

    #[method(name = "get_zprepassFlag", args = 0)]
    pub fn get_zprepass_flag(self) -> bool;

    #[method(name = "set_zprepassFlag", args = 1)]
    pub fn set_zprepass_flag(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> ();

    #[method(name = "EnableCustomViewport", args = 4)]
    pub fn enable_custom_viewport(
        self,
        original_w: f32,
        original_h: f32,
        custom_w: f32,
        custom_h: f32,
    ) -> ();

    #[method(name = "DisableCustomViewport", args = 0)]
    pub fn disable_custom_viewport(self) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customcharaopaquepass")]
impl CustomCharaOpaquePass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomCharaOpaquePass),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomCharaOpaquePassMethods>::ctor(this, evt, layer_mask);
        this
    }
}
