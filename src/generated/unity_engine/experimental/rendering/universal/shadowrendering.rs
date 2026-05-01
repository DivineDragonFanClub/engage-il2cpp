
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/shadowrendering/ShadowRendering.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "ShadowRendering"
)]
#[parent(crate::system::object::Object)]
pub struct ShadowRendering {
    #[static_field]
    #[rename(name = "k_LightPosID")]
    pub k_light_pos_id: i32,
    #[static_field]
    #[rename(name = "k_ShadowStencilGroupID")]
    pub k_shadow_stencil_group_id: i32,
    #[static_field]
    #[rename(name = "k_ShadowIntensityID")]
    pub k_shadow_intensity_id: i32,
    #[static_field]
    #[rename(name = "k_ShadowVolumeIntensityID")]
    pub k_shadow_volume_intensity_id: i32,
    #[static_field]
    #[rename(name = "k_ShadowRadiusID")]
    pub k_shadow_radius_id: i32,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-shadowrendering")]
#[::unity2::methods]
impl ShadowRendering {
    #[method(name = "GetShadowMaterial", args = 2)]
    pub fn get_shadow_material(
        renderer_data : crate :: unity_engine :: experimental :: rendering :: universal :: renderer2ddata :: Renderer2DData,
        index: i32,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "GetRemoveSelfShadowMaterial", args = 2)]
    pub fn get_remove_self_shadow_material(
        renderer_data : crate :: unity_engine :: experimental :: rendering :: universal :: renderer2ddata :: Renderer2DData,
        index: i32,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "CreateShadowRenderTexture", args = 4)]
    pub fn create_shadow_render_texture(
        pass: crate::unity_engine::experimental::rendering::universal::irenderpass2d::IRenderPass2D,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        blend_style_index: i32,
    ) -> ();

    #[method(name = "RenderShadows", args = 8)]
    pub fn render_shadows(
        pass: crate::unity_engine::experimental::rendering::universal::irenderpass2d::IRenderPass2D,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
        cmd_buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        layer_to_render: i32,
        light: crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
        shadow_intensity: f32,
        render_texture : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        depth_texture : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
