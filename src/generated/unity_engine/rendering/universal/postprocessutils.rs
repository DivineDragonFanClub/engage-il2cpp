
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/postprocessutils/PostProcessUtils_ShaderConstants.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "PostProcessUtils.ShaderConstants"
)]
#[parent(crate::system::object::Object)]
pub struct PostProcessUtils_ShaderConstants {
    #[static_field]
    #[rename(name = "_Grain_Texture")]
    pub grain_texture: i32,
    #[static_field]
    #[rename(name = "_Grain_Params")]
    pub grain_params: i32,
    #[static_field]
    #[rename(name = "_Grain_TilingParams")]
    pub grain_tiling_params: i32,
    #[static_field]
    #[rename(name = "_BlueNoise_Texture")]
    pub blue_noise_texture: i32,
    #[static_field]
    #[rename(name = "_Dithering_Params")]
    pub dithering_params: i32,
    #[static_field]
    #[rename(name = "_SourceSize")]
    pub source_size: i32,
}

#[cfg(feature = "unity_engine-rendering-universal-postprocessutils")]
#[::unity2::methods]
impl PostProcessUtils_ShaderConstants {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/postprocessutils/PostProcessUtils.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "PostProcessUtils"
)]
#[parent(crate::system::object::Object)]
pub struct PostProcessUtils {}

#[cfg(feature = "unity_engine-rendering-universal-postprocessutils")]
#[::unity2::methods]
impl PostProcessUtils {
    #[method(name = "ConfigureDithering", args = 4)]
    pub fn configure_dithering(
        data: crate::unity_engine::rendering::universal::postprocessdata::PostProcessData,
        index: i32,
        camera: crate::unity_engine::camera::Camera,
        material: crate::unity_engine::material::Material,
    ) -> i32;

    #[method(name = "ConfigureDithering", args = 5)]
    pub fn configure_dithering_2(
        data: crate::unity_engine::rendering::universal::postprocessdata::PostProcessData,
        index: i32,
        camera_pixel_width: i32,
        camera_pixel_height: i32,
        material: crate::unity_engine::material::Material,
    ) -> i32;

    #[method(name = "ConfigureFilmGrain", args = 4)]
    pub fn configure_film_grain(
        data: crate::unity_engine::rendering::universal::postprocessdata::PostProcessData,
        settings: crate::unity_engine::rendering::universal::filmgrain::FilmGrain,
        camera: crate::unity_engine::camera::Camera,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "ConfigureFilmGrain", args = 5)]
    pub fn configure_film_grain_2(
        data: crate::unity_engine::rendering::universal::postprocessdata::PostProcessData,
        settings: crate::unity_engine::rendering::universal::filmgrain::FilmGrain,
        camera_pixel_width: i32,
        camera_pixel_height: i32,
        material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "SetSourceSize", args = 2)]
    pub fn set_source_size(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();
}
