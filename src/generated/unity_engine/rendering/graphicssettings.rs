
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/graphicssettings/GraphicsSettings.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "GraphicsSettings")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct GraphicsSettings {}

#[cfg(feature = "unity_engine-rendering-graphicssettings")]
#[::unity2::methods]
impl GraphicsSettings {
    #[method(name = "get_lightsUseLinearIntensity", args = 0)]
    pub fn get_lights_use_linear_intensity() -> bool;

    #[method(name = "set_lightsUseLinearIntensity", args = 1)]
    pub fn set_lights_use_linear_intensity(value: bool) -> ();

    #[method(name = "set_useScriptableRenderPipelineBatching", args = 1)]
    pub fn set_use_scriptable_render_pipeline_batching(value: bool) -> ();

    #[method(name = "HasShaderDefine", args = 2)]
    pub fn has_shader_define(
        tier: crate::unity_engine::rendering::graphicstier::GraphicsTier,
        define_hash: crate::unity_engine::rendering::builtinshaderdefine::BuiltinShaderDefine,
    ) -> bool;

    #[method(name = "get_INTERNAL_currentRenderPipeline", args = 0)]
    pub fn get_internal_current_render_pipeline(
    ) -> crate::unity_engine::scriptableobject::ScriptableObject;

    #[method(name = "get_currentRenderPipeline", args = 0)]
    pub fn get_current_render_pipeline(
    ) -> crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset;

    #[method(name = "get_renderPipelineAsset", args = 0)]
    pub fn get_render_pipeline_asset(
    ) -> crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset;

    #[method(name = "set_renderPipelineAsset", args = 1)]
    pub fn set_render_pipeline_asset(
        value: crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset,
    ) -> ();

    #[method(name = "get_INTERNAL_defaultRenderPipeline", args = 0)]
    pub fn get_internal_default_render_pipeline(
    ) -> crate::unity_engine::scriptableobject::ScriptableObject;

    #[method(name = "set_INTERNAL_defaultRenderPipeline", args = 1)]
    pub fn set_internal_default_render_pipeline(
        value: crate::unity_engine::scriptableobject::ScriptableObject,
    ) -> ();

    #[method(name = "get_defaultRenderPipeline", args = 0)]
    pub fn get_default_render_pipeline(
    ) -> crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset;

    #[method(name = "set_defaultRenderPipeline", args = 1)]
    pub fn set_default_render_pipeline(
        value: crate::unity_engine::rendering::renderpipelineasset::RenderPipelineAsset,
    ) -> ();
}
