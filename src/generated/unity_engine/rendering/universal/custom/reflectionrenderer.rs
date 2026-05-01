
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderer::IScriptableRenderer;
use crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/reflectionrenderer/ReflectionRenderer.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom",
    name = "ReflectionRenderer"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer)]
pub struct ReflectionRenderer {
# [static_field] # [rename (name = "k_DepthStencilBufferBits")] pub k_depth_stencil_buffer_bits : i32 ,
# [static_field] # [rename (name = "k_CreateCameraTextures")] pub k_create_camera_textures : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "k_SubLightTag")] pub k_sub_light_tag : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "k_SetupCustomRPConstants")] pub k_setup_custom_rp_constants : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "s_CustomExposurePropID")] pub s_custom_exposure_prop_id : i32 ,
# [static_field] # [rename (name = "s_CustomLodFadeBiasPropID")] pub s_custom_lod_fade_bias_prop_id : i32 ,
# [static_field] # [rename (name = "s_CustomReflectionBaseYPropID")] pub s_custom_reflection_base_y_prop_id : i32 ,
# [static_field] # [rename (name = "s_CustomReflectionBaseYvPropID")] pub s_custom_reflection_base_yv_prop_id : i32 ,
# [rename (name = "m_CustomBaseOpaquePass")] pub m_custom_base_opaque_pass : crate :: unity_engine :: rendering :: universal :: custom :: internal :: custombaseopaquepass :: CustomBaseOpaquePass ,
# [rename (name = "m_CustomCharaOpaquePass")] pub m_custom_chara_opaque_pass : crate :: unity_engine :: rendering :: universal :: custom :: internal :: customcharaopaquepass :: CustomCharaOpaquePass ,
# [rename (name = "m_CustomSpecialReflectionPass")] pub m_custom_special_reflection_pass : crate :: unity_engine :: rendering :: universal :: custom :: internal :: customspecialreflectionpass :: CustomSpecialReflectionPass ,
# [rename (name = "m_UseCustomBaseOpaquePass")] pub m_use_custom_base_opaque_pass : bool ,
# [rename (name = "m_UseSpecialReflectionPass")] pub m_use_special_reflection_pass : bool ,
# [rename (name = "m_ActiveCameraColorAttachment")] pub m_active_camera_color_attachment : crate :: unity_engine :: rendering :: universal :: rendertargethandle :: RenderTargetHandle ,
# [rename (name = "m_ActiveCameraDepthAttachment")] pub m_active_camera_depth_attachment : crate :: unity_engine :: rendering :: universal :: rendertargethandle :: RenderTargetHandle ,
# [rename (name = "m_ForwardLights")] pub m_forward_lights : crate :: unity_engine :: rendering :: universal :: internal :: forwardlights :: ForwardLights ,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-reflectionrenderer")]
#[::unity2::methods]
impl ReflectionRenderer {
    #[method(name = "get_reflectionBaseY", args = 0)]
    pub fn get_reflection_base_y() -> f32;

    #[method(name = "set_reflectionBaseY", args = 1)]
    pub fn set_reflection_base_y(value: f32) -> ();

    #[method(name = "get_reflectionBaseY_Chara", args = 0)]
    pub fn get_reflection_base_y_chara() -> f32;

    #[method(name = "set_reflectionBaseY_Chara", args = 1)]
    pub fn set_reflection_base_y_chara(value: f32) -> ();

    #[method(name = "get_reflectionBaseY_Map", args = 0)]
    pub fn get_reflection_base_y_map() -> f32;

    #[method(name = "set_reflectionBaseY_Map", args = 1)]
    pub fn set_reflection_base_y_map(value: f32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        data : crate :: unity_engine :: rendering :: universal :: custom :: reflectionrendererdata :: ReflectionRendererData,
    ) -> ();

    #[method(name = "GetSubLightTag", args = 0)]
    pub fn get_sub_light_tag(self) -> ::unity2::Il2CppString;

    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "SetupLights", args = 2)]
    pub fn setup_lights(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "SetupCullingParameters", args = 2)]
    pub fn setup_culling_parameters(
        self,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
    ) -> ();

    #[method(name = "FinishRendering", args = 1)]
    pub fn finish_rendering(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-reflectionrenderer")]
impl ReflectionRenderer {
    pub fn new(
        data : crate :: unity_engine :: rendering :: universal :: custom :: reflectionrendererdata :: ReflectionRendererData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionRendererMethods>::ctor(this, data);
        this
    }
}
