
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::rendering::renderpipelineasset::IRenderPipelineAsset;
use crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/universalrenderpipelineasset/UniversalRenderPipelineAsset.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "UniversalRenderPipelineAsset"
)]
#[parent(crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset)]
pub struct UniversalRenderPipelineAsset {
    #[rename(name = "m_DefaultShader")]
    pub m_default_shader: crate::unity_engine::shader::Shader,
    #[rename(name = "m_Renderers")]
    pub m_renderers: ::unity2::Array<
        crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer,
    >,
    #[rename(name = "k_AssetVersion")]
    pub k_asset_version: i32,
    #[rename(name = "k_AssetPreviousVersion")]
    pub k_asset_previous_version: i32,
    #[rename(name = "m_RendererType")]
    pub m_renderer_type: crate::unity_engine::rendering::universal::renderertype::RendererType,
    #[rename(name = "m_RendererData")]
    pub m_renderer_data:
        crate::unity_engine::rendering::universal::scriptablerendererdata::ScriptableRendererData,
    #[rename(name = "m_RendererDataList")]
    pub m_renderer_data_list: ::unity2::Array<
        crate::unity_engine::rendering::universal::scriptablerendererdata::ScriptableRendererData,
    >,
    #[rename(name = "m_DefaultRendererIndex")]
    pub m_default_renderer_index: i32,
    #[rename(name = "m_RequireDepthTexture")]
    pub m_require_depth_texture: bool,
    #[rename(name = "m_RequireOpaqueTexture")]
    pub m_require_opaque_texture: bool,
    #[rename(name = "m_OpaqueDownsampling")]
    pub m_opaque_downsampling:
        crate::unity_engine::rendering::universal::downsampling::Downsampling,
    #[rename(name = "m_SupportsTerrainHoles")]
    pub m_supports_terrain_holes: bool,
    #[rename(name = "m_SupportsHDR")]
    pub m_supports_hdr: bool,
    #[rename(name = "m_MSAA")]
    pub m_msaa: crate::unity_engine::rendering::universal::msaaquality::MsaaQuality,
    #[rename(name = "m_RenderScale")]
    pub m_render_scale: f32,
    #[rename(name = "m_MainLightRenderingMode")]
    pub m_main_light_rendering_mode:
        crate::unity_engine::rendering::universal::lightrenderingmode::LightRenderingMode,
    #[rename(name = "m_MainLightShadowsSupported")]
    pub m_main_light_shadows_supported: bool,
    #[rename(name = "m_MainLightShadowmapResolution")]
    pub m_main_light_shadowmap_resolution:
        crate::unity_engine::rendering::universal::shadowresolution_2::ShadowResolution_2,
    #[rename(name = "m_AdditionalLightsRenderingMode")]
    pub m_additional_lights_rendering_mode:
        crate::unity_engine::rendering::universal::lightrenderingmode::LightRenderingMode,
    #[rename(name = "m_AdditionalLightsPerObjectLimit")]
    pub m_additional_lights_per_object_limit: i32,
    #[rename(name = "m_AdditionalLightShadowsSupported")]
    pub m_additional_light_shadows_supported: bool,
    #[rename(name = "m_AdditionalLightsShadowmapResolution")]
    pub m_additional_lights_shadowmap_resolution:
        crate::unity_engine::rendering::universal::shadowresolution_2::ShadowResolution_2,
    #[rename(name = "m_ShadowDistance")]
    pub m_shadow_distance: f32,
    #[rename(name = "m_ShadowCascadeCount")]
    pub m_shadow_cascade_count: i32,
    #[rename(name = "m_Cascade2Split")]
    pub m_cascade2_split: f32,
    #[rename(name = "m_Cascade3Split")]
    pub m_cascade3_split: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Cascade4Split")]
    pub m_cascade4_split: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_ShadowDepthBias")]
    pub m_shadow_depth_bias: f32,
    #[rename(name = "m_ShadowNormalBias")]
    pub m_shadow_normal_bias: f32,
    #[rename(name = "m_SoftShadowsSupported")]
    pub m_soft_shadows_supported: bool,
    #[rename(name = "m_UseSRPBatcher")]
    pub m_use_srp_batcher: bool,
    #[rename(name = "m_SupportsDynamicBatching")]
    pub m_supports_dynamic_batching: bool,
    #[rename(name = "m_MixedLightingSupported")]
    pub m_mixed_lighting_supported: bool,
    #[rename(name = "m_DebugLevel")]
    pub m_debug_level:
        crate::unity_engine::rendering::universal::pipelinedebuglevel::PipelineDebugLevel,
    #[rename(name = "m_UseAdaptivePerformance")]
    pub m_use_adaptive_performance: bool,
    #[rename(name = "m_ColorGradingMode")]
    pub m_color_grading_mode:
        crate::unity_engine::rendering::universal::colorgradingmode::ColorGradingMode,
    #[rename(name = "m_ColorGradingLutSize")]
    pub m_color_grading_lut_size: i32,
    #[rename(name = "m_ShadowType")]
    pub m_shadow_type: crate::unity_engine::rendering::universal::shadowquality::ShadowQuality,
    #[rename(name = "m_LocalShadowsSupported")]
    pub m_local_shadows_supported: bool,
    #[rename(name = "m_LocalShadowsAtlasResolution")]
    pub m_local_shadows_atlas_resolution:
        crate::unity_engine::rendering::universal::shadowresolution_2::ShadowResolution_2,
    #[rename(name = "m_MaxPixelLights")]
    pub m_max_pixel_lights: i32,
    #[rename(name = "m_ShadowAtlasResolution")]
    pub m_shadow_atlas_resolution:
        crate::unity_engine::rendering::universal::shadowresolution_2::ShadowResolution_2,
    #[rename(name = "m_ShaderVariantLogLevel")]
    pub m_shader_variant_log_level:
        crate::unity_engine::rendering::universal::shadervariantloglevel::ShaderVariantLogLevel,
    #[rename(name = "m_CustomShadowDataList")]
    pub m_custom_shadow_data_list: ::unity2::Array<
        crate::unity_engine::rendering::universal::custom::customshadowdata::CustomShadowData,
    >,
    #[rename(name = "m_SharpenFilter")]
    pub m_sharpen_filter: bool,
    #[static_field]
    #[rename(name = "k_MinLutSize")]
    pub k_min_lut_size: i32,
    #[static_field]
    #[rename(name = "k_MaxLutSize")]
    pub k_max_lut_size: i32,
    #[static_field]
    #[rename(name = "k_ShadowCascadeMinCount")]
    pub k_shadow_cascade_min_count: i32,
    #[static_field]
    #[rename(name = "k_ShadowCascadeMaxCount")]
    pub k_shadow_cascade_max_count: i32,
    #[rename(name = "m_ShadowCascades")]
    pub m_shadow_cascades:
        crate::unity_engine::rendering::universal::shadowcascadesoption::ShadowCascadesOption,
}

#[cfg(feature = "unity_engine-rendering-universal-universalrenderpipelineasset")]
#[::unity2::methods]
impl UniversalRenderPipelineAsset {
    #[method(name = "get_sharpenFilter", args = 0)]
    pub fn get_sharpen_filter(self) -> bool;

    #[method(name = "set_sharpenFilter", args = 1)]
    pub fn set_sharpen_filter(self, value: bool) -> ();

    #[method(name = "get_lutPatternV", args = 0)]
    pub fn get_lut_pattern_v(self) -> f32;

    #[method(name = "set_lutPatternV", args = 1)]
    pub fn set_lut_pattern_v(self, value: f32) -> ();

    #[method(name = "CalcLutPatternParams", args = 2)]
    pub fn calc_lut_pattern_params(self, pattern_index: i32, pattern_count: i32) -> ();

    #[method(name = "get_customAlphaMapUI", args = 0)]
    pub fn get_custom_alpha_map_ui(self) -> f32;

    #[method(name = "set_customAlphaMapUI", args = 1)]
    pub fn set_custom_alpha_map_ui(self, value: f32) -> ();

    #[method(name = "get_customRenderingFlag", args = 0)]
    pub fn get_custom_rendering_flag(
        self,
    ) -> crate::unity_engine::rendering::universal::customrenderingflag::CustomRenderingFlag;

    #[method(name = "set_customRenderingFlag", args = 1)]
    pub fn set_custom_rendering_flag(
        self,
        value: crate::unity_engine::rendering::universal::customrenderingflag::CustomRenderingFlag,
    ) -> ();

    #[method(name = "LoadBuiltinRendererData", args = 1)]
    pub fn load_builtin_renderer_data(
        self,
        r#type: crate::unity_engine::rendering::universal::renderertype::RendererType,
    ) -> crate::unity_engine::rendering::universal::scriptablerendererdata::ScriptableRendererData;

    #[method(name = "CreatePipeline", args = 0)]
    pub fn create_pipeline(self) -> crate::unity_engine::rendering::renderpipeline::RenderPipeline;

    #[method(name = "DestroyRenderers", args = 0)]
    pub fn destroy_renderers(self) -> ();

    #[method(name = "DestroyRenderer", args = 1)]
    pub fn destroy_renderer(
        self,
        renderer: crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer,
    ) -> ();

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "CreateRenderers", args = 0)]
    pub fn create_renderers(self) -> ();

    #[method(name = "GetMaterial", args = 1)]
    pub fn get_material(
        self,
        material_type : crate :: unity_engine :: rendering :: universal :: defaultmaterialtype :: DefaultMaterialType,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "get_scriptableRenderer", args = 0)]
    pub fn get_scriptable_renderer(
        self,
    ) -> crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;

    #[method(name = "GetRenderer", args = 1)]
    pub fn get_renderer(
        self,
        index: i32,
    ) -> crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;

    #[method(name = "get_scriptableRendererData", args = 0)]
    pub fn get_scriptable_renderer_data(
        self,
    ) -> crate::unity_engine::rendering::universal::scriptablerendererdata::ScriptableRendererData;

    #[method(name = "get_rendererIndexList", args = 0)]
    pub fn get_renderer_index_list(self) -> ::unity2::Array<i32>;

    #[method(name = "get_supportsCameraDepthTexture", args = 0)]
    pub fn get_supports_camera_depth_texture(self) -> bool;

    #[method(name = "set_supportsCameraDepthTexture", args = 1)]
    pub fn set_supports_camera_depth_texture(self, value: bool) -> ();

    #[method(name = "get_supportsCameraOpaqueTexture", args = 0)]
    pub fn get_supports_camera_opaque_texture(self) -> bool;

    #[method(name = "set_supportsCameraOpaqueTexture", args = 1)]
    pub fn set_supports_camera_opaque_texture(self, value: bool) -> ();

    #[method(name = "get_opaqueDownsampling", args = 0)]
    pub fn get_opaque_downsampling(
        self,
    ) -> crate::unity_engine::rendering::universal::downsampling::Downsampling;

    #[method(name = "get_supportsTerrainHoles", args = 0)]
    pub fn get_supports_terrain_holes(self) -> bool;

    #[method(name = "get_supportsHDR", args = 0)]
    pub fn get_supports_hdr(self) -> bool;

    #[method(name = "set_supportsHDR", args = 1)]
    pub fn set_supports_hdr(self, value: bool) -> ();

    #[method(name = "get_msaaSampleCount", args = 0)]
    pub fn get_msaa_sample_count(self) -> i32;

    #[method(name = "set_msaaSampleCount", args = 1)]
    pub fn set_msaa_sample_count(self, value: i32) -> ();

    #[method(name = "get_renderScale", args = 0)]
    pub fn get_render_scale(self) -> f32;

    #[method(name = "set_renderScale", args = 1)]
    pub fn set_render_scale(self, value: f32) -> ();

    #[method(name = "get_mainLightRenderingMode", args = 0)]
    pub fn get_main_light_rendering_mode(
        self,
    ) -> crate::unity_engine::rendering::universal::lightrenderingmode::LightRenderingMode;

    #[method(name = "get_supportsMainLightShadows", args = 0)]
    pub fn get_supports_main_light_shadows(self) -> bool;

    #[method(name = "get_mainLightShadowmapResolution", args = 0)]
    pub fn get_main_light_shadowmap_resolution(self) -> i32;

    #[method(name = "get_additionalLightsRenderingMode", args = 0)]
    pub fn get_additional_lights_rendering_mode(
        self,
    ) -> crate::unity_engine::rendering::universal::lightrenderingmode::LightRenderingMode;

    #[method(name = "get_maxAdditionalLightsCount", args = 0)]
    pub fn get_max_additional_lights_count(self) -> i32;

    #[method(name = "set_maxAdditionalLightsCount", args = 1)]
    pub fn set_max_additional_lights_count(self, value: i32) -> ();

    #[method(name = "get_supportsAdditionalLightShadows", args = 0)]
    pub fn get_supports_additional_light_shadows(self) -> bool;

    #[method(name = "get_additionalLightsShadowmapResolution", args = 0)]
    pub fn get_additional_lights_shadowmap_resolution(self) -> i32;

    #[method(name = "get_shadowDistance", args = 0)]
    pub fn get_shadow_distance(self) -> f32;

    #[method(name = "set_shadowDistance", args = 1)]
    pub fn set_shadow_distance(self, value: f32) -> ();

    #[method(name = "get_shadowCascadeCount", args = 0)]
    pub fn get_shadow_cascade_count(self) -> i32;

    #[method(name = "set_shadowCascadeCount", args = 1)]
    pub fn set_shadow_cascade_count(self, value: i32) -> ();

    #[method(name = "get_cascade2Split", args = 0)]
    pub fn get_cascade2_split(self) -> f32;

    #[method(name = "get_cascade3Split", args = 0)]
    pub fn get_cascade3_split(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_cascade4Split", args = 0)]
    pub fn get_cascade4_split(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_shadowDepthBias", args = 0)]
    pub fn get_shadow_depth_bias(self) -> f32;

    #[method(name = "set_shadowDepthBias", args = 1)]
    pub fn set_shadow_depth_bias(self, value: f32) -> ();

    #[method(name = "get_shadowNormalBias", args = 0)]
    pub fn get_shadow_normal_bias(self) -> f32;

    #[method(name = "set_shadowNormalBias", args = 1)]
    pub fn set_shadow_normal_bias(self, value: f32) -> ();

    #[method(name = "get_supportsSoftShadows", args = 0)]
    pub fn get_supports_soft_shadows(self) -> bool;

    #[method(name = "get_supportsDynamicBatching", args = 0)]
    pub fn get_supports_dynamic_batching(self) -> bool;

    #[method(name = "set_supportsDynamicBatching", args = 1)]
    pub fn set_supports_dynamic_batching(self, value: bool) -> ();

    #[method(name = "get_supportsMixedLighting", args = 0)]
    pub fn get_supports_mixed_lighting(self) -> bool;

    #[method(name = "get_shaderVariantLogLevel", args = 0)]
    pub fn get_shader_variant_log_level(
        self,
    ) -> crate::unity_engine::rendering::universal::shadervariantloglevel::ShaderVariantLogLevel;

    #[method(name = "set_shaderVariantLogLevel", args = 1)]
    pub fn set_shader_variant_log_level(
        self,
        value : crate :: unity_engine :: rendering :: universal :: shadervariantloglevel :: ShaderVariantLogLevel,
    ) -> ();

    #[method(name = "get_debugLevel", args = 0)]
    pub fn get_debug_level(
        self,
    ) -> crate::unity_engine::rendering::universal::pipelinedebuglevel::PipelineDebugLevel;

    #[method(name = "set_debugLevel", args = 1)]
    pub fn set_debug_level(
        self,
        value: crate::unity_engine::rendering::universal::pipelinedebuglevel::PipelineDebugLevel,
    ) -> ();

    #[method(name = "get_useSRPBatcher", args = 0)]
    pub fn get_use_srp_batcher(self) -> bool;

    #[method(name = "set_useSRPBatcher", args = 1)]
    pub fn set_use_srp_batcher(self, value: bool) -> ();

    #[method(name = "get_colorGradingMode", args = 0)]
    pub fn get_color_grading_mode(
        self,
    ) -> crate::unity_engine::rendering::universal::colorgradingmode::ColorGradingMode;

    #[method(name = "set_colorGradingMode", args = 1)]
    pub fn set_color_grading_mode(
        self,
        value: crate::unity_engine::rendering::universal::colorgradingmode::ColorGradingMode,
    ) -> ();

    #[method(name = "get_colorGradingLutSize", args = 0)]
    pub fn get_color_grading_lut_size(self) -> i32;

    #[method(name = "set_colorGradingLutSize", args = 1)]
    pub fn set_color_grading_lut_size(self, value: i32) -> ();

    #[method(name = "get_useAdaptivePerformance", args = 0)]
    pub fn get_use_adaptive_performance(self) -> bool;

    #[method(name = "set_useAdaptivePerformance", args = 1)]
    pub fn set_use_adaptive_performance(self, value: bool) -> ();

    #[method(name = "get_defaultMaterial", args = 0)]
    pub fn get_default_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultParticleMaterial", args = 0)]
    pub fn get_default_particle_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultLineMaterial", args = 0)]
    pub fn get_default_line_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultTerrainMaterial", args = 0)]
    pub fn get_default_terrain_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultUIMaterial", args = 0)]
    pub fn get_default_ui_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultUIOverdrawMaterial", args = 0)]
    pub fn get_default_ui_overdraw_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultUIETC1SupportedMaterial", args = 0)]
    pub fn get_default_uietc1_supported_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_default2DMaterial", args = 0)]
    pub fn get_default2_d_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_defaultShader", args = 0)]
    pub fn get_default_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "OnBeforeSerialize", args = 0)]
    pub fn on_before_serialize(self) -> ();

    #[method(name = "OnAfterDeserialize", args = 0)]
    pub fn on_after_deserialize(self) -> ();

    #[method(name = "ValidateShadowBias", args = 1)]
    pub fn validate_shadow_bias(self, value: f32) -> f32;

    #[method(name = "ValidatePerObjectLights", args = 1)]
    pub fn validate_per_object_lights(self, value: i32) -> i32;

    #[method(name = "ValidateRenderScale", args = 1)]
    pub fn validate_render_scale(self, value: f32) -> f32;

    #[method(name = "ValidateRendererDataList", args = 1)]
    pub fn validate_renderer_data_list(self, partial: bool) -> bool;

    #[method(name = "ValidateRendererData", args = 1)]
    pub fn validate_renderer_data(self, index: i32) -> bool;

    #[method(name = "GetCustomShadowData", args = 1)]
    pub fn get_custom_shadow_data(
        self,
        index: i32,
    ) -> crate::unity_engine::rendering::universal::custom::customshadowdata::CustomShadowData;

    #[method(name = "ApplyCustomShadowData", args = 1)]
    pub fn apply_custom_shadow_data(
        self,
        data: crate::unity_engine::rendering::universal::custom::customshadowdata::CustomShadowData,
    ) -> ();

    #[method(name = "ApplyCustomShadowData", args = 3)]
    pub fn apply_custom_shadow_data_2(
        self,
        data_a : crate :: unity_engine :: rendering :: universal :: custom :: customshadowdata :: CustomShadowData,
        data_b : crate :: unity_engine :: rendering :: universal :: custom :: customshadowdata :: CustomShadowData,
        t: f32,
    ) -> ();

    #[method(name = "UpdateShadowCascade2Split", args = 1)]
    pub fn update_shadow_cascade2_split(self, split1: f32) -> ();

    #[method(name = "get_shadowCascadeOption", args = 0)]
    pub fn get_shadow_cascade_option(
        self,
    ) -> crate::unity_engine::rendering::universal::shadowcascadesoption::ShadowCascadesOption;

    #[method(name = "set_shadowCascadeOption", args = 1)]
    pub fn set_shadow_cascade_option(
        self,
        value : crate :: unity_engine :: rendering :: universal :: shadowcascadesoption :: ShadowCascadesOption,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-universalrenderpipelineasset")]
impl UniversalRenderPipelineAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UniversalRenderPipelineAsset),
                ::core::stringify!(new),
            )
        });
        <Self as IUniversalRenderPipelineAssetMethods>::ctor(this);
        this
    }
}
