
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/additionallightsshadowcasterpass/AdditionalLightsShadowCasterPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "AdditionalLightsShadowCasterPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct AdditionalLightsShadowCasterPass {
    #[static_field]
    #[rename(name = "m_AdditionalShadowsBufferId")]
    pub m_additional_shadows_buffer_id: i32,
    #[static_field]
    #[rename(name = "m_AdditionalShadowsIndicesId")]
    pub m_additional_shadows_indices_id: i32,
    #[rename(name = "m_UseStructuredBuffer")]
    pub m_use_structured_buffer: bool,
    #[static_field]
    #[rename(name = "k_ShadowmapBufferBits")]
    pub k_shadowmap_buffer_bits: i32,
    #[rename(name = "m_AdditionalLightsShadowmap")]
    pub m_additional_lights_shadowmap:
        crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[rename(name = "m_AdditionalLightsShadowmapTexture")]
    pub m_additional_lights_shadowmap_texture: crate::unity_engine::rendertexture::RenderTexture,
    #[rename(name = "m_ShadowmapWidth")]
    pub m_shadowmap_width: i32,
    #[rename(name = "m_ShadowmapHeight")]
    pub m_shadowmap_height: i32,
    #[rename(name = "m_AdditionalLightSlices")]
    pub m_additional_light_slices: ::unity2::Array<
        crate::unity_engine::rendering::universal::shadowslicedata::ShadowSliceData,
    >,
    #[rename(name = "m_AdditionalLightsWorldToShadow")]
    pub m_additional_lights_world_to_shadow:
        ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "m_AdditionalLightsShadowParams")]
    pub m_additional_lights_shadow_params: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_AdditionalLightsShadowData")]
    pub m_additional_lights_shadow_data: ::unity2::Array<
        crate::unity_engine::rendering::universal::shaderinput::ShaderInput_ShadowData,
    >,
    #[rename(name = "m_AdditionalShadowCastingLightIndices")]
    pub m_additional_shadow_casting_light_indices:
        crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_AdditionalShadowCastingLightIndicesMap")]
    pub m_additional_shadow_casting_light_indices_map:
        crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_ShadowCastingLightIndicesMap")]
    pub m_shadow_casting_light_indices_map:
        crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_SupportsBoxFilterForShadows")]
    pub m_supports_box_filter_for_shadows: bool,
    #[rename(name = "m_ProfilingSetupSampler")]
    pub m_profiling_setup_sampler:
        crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
}

#[cfg(feature = "unity_engine-rendering-universal-internal-additionallightsshadowcasterpass")]
#[::unity2::methods]
impl AdditionalLightsShadowCasterPass {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(
        self,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> bool;

    #[method(name = "Configure", args = 2)]
    pub fn configure(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera_texture_descriptor : crate :: unity_engine :: rendertexturedescriptor :: RenderTextureDescriptor,
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

    #[method(name = "GetShadowLightIndexFromLightIndex", args = 1)]
    pub fn get_shadow_light_index_from_light_index(self, visible_light_index: i32) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "RenderAdditionalShadowmapAtlas", args = 4)]
    pub fn render_additional_shadowmap_atlas(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        light_data: crate::unity_engine::rendering::universal::lightdata::LightData,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
    ) -> ();

    #[method(name = "SetupAdditionalLightsShadowReceiverConstants", args = 3)]
    pub fn setup_additional_lights_shadow_receiver_constants(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
        soft_shadows: bool,
    ) -> ();

    #[method(name = "IsValidShadowCastingLight", args = 2)]
    pub fn is_valid_shadow_casting_light(
        self,
        light_data: crate::unity_engine::rendering::universal::lightdata::LightData,
        i: i32,
    ) -> bool;
}

#[cfg(feature = "unity_engine-rendering-universal-internal-additionallightsshadowcasterpass")]
impl AdditionalLightsShadowCasterPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AdditionalLightsShadowCasterPass),
                ::core::stringify!(new),
            )
        });
        <Self as IAdditionalLightsShadowCasterPassMethods>::ctor(this, evt);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/additionallightsshadowcasterpass/AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "AdditionalLightsShadowCasterPass.AdditionalShadowsConstantBuffer"
)]
#[parent(crate::system::object::Object)]
pub struct AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    #[static_field]
    #[rename(name = "_AdditionalLightsWorldToShadow")]
    pub additional_lights_world_to_shadow: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowParams")]
    pub additional_shadow_params: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowOffset0")]
    pub additional_shadow_offset0: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowOffset1")]
    pub additional_shadow_offset1: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowOffset2")]
    pub additional_shadow_offset2: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowOffset3")]
    pub additional_shadow_offset3: i32,
    #[static_field]
    #[rename(name = "_AdditionalShadowmapSize")]
    pub additional_shadowmap_size: i32,
}
