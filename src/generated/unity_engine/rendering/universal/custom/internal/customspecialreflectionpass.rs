
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/internal/customspecialreflectionpass/CustomSpecialReflectionPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom.Internal",
    name = "CustomSpecialReflectionPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CustomSpecialReflectionPass {
    #[rename(name = "m_FilteringSettings")]
    pub m_filtering_settings: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    #[rename(name = "m_ShaderTagId")]
    pub m_shader_tag_id: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    #[rename(name = "m_ProfilingSampler")]
    pub m_profiling_sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[static_field]
    #[rename(name = "s_DrawObjectPassDataPropID")]
    pub s_draw_object_pass_data_prop_id: i32,
    #[rename(name = "m_CustomViewport")]
    pub m_custom_viewport: bool,
    #[rename(name = "m_CustomViewportRect")]
    pub m_custom_viewport_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_OriginalViewportRect")]
    pub m_original_viewport_rect: crate::unity_engine::rect::Rect,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customspecialreflectionpass")]
#[::unity2::methods]
impl CustomSpecialReflectionPass {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        profiler_tag: ::unity2::Il2CppString,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        render_queue_range: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
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

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customspecialreflectionpass")]
impl CustomSpecialReflectionPass {
    pub fn new(
        profiler_tag: ::unity2::Il2CppString,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        render_queue_range: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomSpecialReflectionPass),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomSpecialReflectionPassMethods>::ctor(
            this,
            profiler_tag,
            evt,
            render_queue_range,
            layer_mask,
        );
        this
    }
}
