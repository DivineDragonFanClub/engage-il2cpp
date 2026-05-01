
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/renderpipelineasset/RenderPipelineAsset.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "RenderPipelineAsset")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct RenderPipelineAsset {}

#[cfg(feature = "unity_engine-rendering-renderpipelineasset")]
#[::unity2::methods]
impl RenderPipelineAsset {
    #[method(name = "InternalCreatePipeline", args = 0)]
    pub fn internal_create_pipeline(
        self,
    ) -> crate::unity_engine::rendering::renderpipeline::RenderPipeline;

    #[method(name = "get_renderingLayerMaskNames", args = 0)]
    pub fn get_rendering_layer_mask_names(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_defaultMaterial", args = 0)]
    pub fn get_default_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_autodeskInteractiveShader", args = 0)]
    pub fn get_autodesk_interactive_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_autodeskInteractiveTransparentShader", args = 0)]
    pub fn get_autodesk_interactive_transparent_shader(self)
        -> crate::unity_engine::shader::Shader;

    #[method(name = "get_autodeskInteractiveMaskedShader", args = 0)]
    pub fn get_autodesk_interactive_masked_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_terrainDetailLitShader", args = 0)]
    pub fn get_terrain_detail_lit_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_terrainDetailGrassShader", args = 0)]
    pub fn get_terrain_detail_grass_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_terrainDetailGrassBillboardShader", args = 0)]
    pub fn get_terrain_detail_grass_billboard_shader(self) -> crate::unity_engine::shader::Shader;

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

    #[method(name = "get_defaultSpeedTree7Shader", args = 0)]
    pub fn get_default_speed_tree7_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_defaultSpeedTree8Shader", args = 0)]
    pub fn get_default_speed_tree8_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "CreatePipeline", args = 0)]
    pub fn create_pipeline(self) -> crate::unity_engine::rendering::renderpipeline::RenderPipeline;

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-renderpipelineasset")]
impl RenderPipelineAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderPipelineAsset),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderPipelineAssetMethods>::ctor(this);
        this
    }
}
