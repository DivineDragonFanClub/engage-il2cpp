
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/postprocessdata/PostProcessData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "PostProcessData"
)]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct PostProcessData {
# [rename (name = "shaders")] pub shaders : crate :: unity_engine :: rendering :: universal :: postprocessdata :: PostProcessData_ShaderResources ,
# [rename (name = "textures")] pub textures : crate :: unity_engine :: rendering :: universal :: postprocessdata :: PostProcessData_TextureResources ,
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
#[::unity2::methods]
impl PostProcessData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
impl PostProcessData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PostProcessData),
                ::core::stringify!(new),
            )
        });
        <Self as IPostProcessDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/postprocessdata/PostProcessData_ShaderResources.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "PostProcessData.ShaderResources"
)]
#[parent(crate::system::object::Object)]
pub struct PostProcessData_ShaderResources {
    #[rename(name = "stopNanPS")]
    pub stop_nan_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "subpixelMorphologicalAntialiasingPS")]
    pub subpixel_morphological_antialiasing_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "gaussianDepthOfFieldPS")]
    pub gaussian_depth_of_field_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "bokehDepthOfFieldPS")]
    pub bokeh_depth_of_field_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "cameraMotionBlurPS")]
    pub camera_motion_blur_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "paniniProjectionPS")]
    pub panini_projection_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "lutBuilderLdrPS")]
    pub lut_builder_ldr_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "lutBuilderHdrPS")]
    pub lut_builder_hdr_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "bloomPS")]
    pub bloom_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "uberPostPS")]
    pub uber_post_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "finalPostPassPS")]
    pub final_post_pass_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "customBlurPS")]
    pub custom_blur_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "customFilterPS")]
    pub custom_filter_ps: crate::unity_engine::shader::Shader,
    #[rename(name = "customUberPostPS")]
    pub custom_uber_post_ps: crate::unity_engine::shader::Shader,
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
#[::unity2::methods]
impl PostProcessData_ShaderResources {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
impl PostProcessData_ShaderResources {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PostProcessData_ShaderResources),
                ::core::stringify!(new),
            )
        });
        <Self as IPostProcessData_ShaderResourcesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/postprocessdata/PostProcessData_TextureResources.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "PostProcessData.TextureResources"
)]
#[parent(crate::system::object::Object)]
pub struct PostProcessData_TextureResources {
    #[rename(name = "blueNoise16LTex")]
    pub blue_noise16_l_tex: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
    #[rename(name = "filmGrainTex")]
    pub film_grain_tex: ::unity2::Array<crate::unity_engine::texture2d::Texture2D>,
    #[rename(name = "smaaAreaTex")]
    pub smaa_area_tex: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "smaaSearchTex")]
    pub smaa_search_tex: crate::unity_engine::texture2d::Texture2D,
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
#[::unity2::methods]
impl PostProcessData_TextureResources {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessdata")]
impl PostProcessData_TextureResources {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PostProcessData_TextureResources),
                ::core::stringify!(new),
            )
        });
        <Self as IPostProcessData_TextureResourcesMethods>::ctor(this);
        this
    }
}
