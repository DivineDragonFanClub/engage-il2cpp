
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/universaladditionalcameradata/UniversalAdditionalCameraData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "UniversalAdditionalCameraData"
)]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UniversalAdditionalCameraData {
# [rename (name = "m_RenderShadows")] pub m_render_shadows : bool ,
# [rename (name = "m_RequiresDepthTextureOption")] pub m_requires_depth_texture_option : crate :: unity_engine :: rendering :: universal :: cameraoverrideoption :: CameraOverrideOption ,
# [rename (name = "m_RequiresOpaqueTextureOption")] pub m_requires_opaque_texture_option : crate :: unity_engine :: rendering :: universal :: cameraoverrideoption :: CameraOverrideOption ,
# [rename (name = "m_CameraType")] pub m_camera_type : crate :: unity_engine :: rendering :: universal :: camerarendertype :: CameraRenderType ,
# [rename (name = "m_Cameras")] pub m_cameras : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: camera :: Camera > ,
# [rename (name = "m_RendererIndex")] pub m_renderer_index : i32 ,
# [rename (name = "m_VolumeLayerMask")] pub m_volume_layer_mask : crate :: unity_engine :: layermask :: LayerMask ,
# [rename (name = "m_VolumeTrigger")] pub m_volume_trigger : crate :: unity_engine :: transform :: Transform ,
# [rename (name = "m_RenderPostProcessing")] pub m_render_post_processing : bool ,
# [rename (name = "m_Antialiasing")] pub m_antialiasing : crate :: unity_engine :: rendering :: universal :: antialiasingmode :: AntialiasingMode ,
# [rename (name = "m_AntialiasingQuality")] pub m_antialiasing_quality : crate :: unity_engine :: rendering :: universal :: antialiasingquality :: AntialiasingQuality ,
# [rename (name = "m_StopNaN")] pub m_stop_na_n : bool ,
# [rename (name = "m_Dithering")] pub m_dithering : bool ,
# [rename (name = "m_ClearDepth")] pub m_clear_depth : bool ,
# [rename (name = "m_AllowXRRendering")] pub m_allow_xr_rendering : bool ,
# [rename (name = "m_IsAutoReduction")] pub m_is_auto_reduction : bool ,
# [rename (name = "m_RequiresDepthTexture")] pub m_requires_depth_texture : bool ,
# [rename (name = "m_RequiresColorTexture")] pub m_requires_color_texture : bool ,
# [rename (name = "m_Version")] pub m_version : f32 ,
# [static_field] # [rename (name = "s_DefaultAdditionalCameraData")] pub s_default_additional_camera_data : crate :: unity_engine :: rendering :: universal :: universaladditionalcameradata :: UniversalAdditionalCameraData ,
# [rename (name = "m_CustomRenderingFlag")] pub m_custom_rendering_flag : crate :: unity_engine :: rendering :: universal :: customcamerarenderingflag :: CustomCameraRenderingFlag ,
}

#[cfg(feature = "unity_engine-rendering-universal-universaladditionalcameradata")]
#[::unity2::methods]
impl UniversalAdditionalCameraData {
    #[method(name = "get_version", args = 0)]
    pub fn get_version(self) -> f32;

    #[method(name = "get_defaultAdditionalCameraData", args = 0)]
    pub fn get_default_additional_camera_data () -> crate :: unity_engine :: rendering :: universal :: universaladditionalcameradata :: UniversalAdditionalCameraData ;

    #[method(name = "get_renderShadows", args = 0)]
    pub fn get_render_shadows(self) -> bool;

    #[method(name = "set_renderShadows", args = 1)]
    pub fn set_render_shadows(self, value: bool) -> ();

    #[method(name = "get_requiresDepthOption", args = 0)]
    pub fn get_requires_depth_option(
        self,
    ) -> crate::unity_engine::rendering::universal::cameraoverrideoption::CameraOverrideOption;

    #[method(name = "set_requiresDepthOption", args = 1)]
    pub fn set_requires_depth_option(
        self,
        value : crate :: unity_engine :: rendering :: universal :: cameraoverrideoption :: CameraOverrideOption,
    ) -> ();

    #[method(name = "get_requiresColorOption", args = 0)]
    pub fn get_requires_color_option(
        self,
    ) -> crate::unity_engine::rendering::universal::cameraoverrideoption::CameraOverrideOption;

    #[method(name = "set_requiresColorOption", args = 1)]
    pub fn set_requires_color_option(
        self,
        value : crate :: unity_engine :: rendering :: universal :: cameraoverrideoption :: CameraOverrideOption,
    ) -> ();

    #[method(name = "get_renderType", args = 0)]
    pub fn get_render_type(
        self,
    ) -> crate::unity_engine::rendering::universal::camerarendertype::CameraRenderType;

    #[method(name = "set_renderType", args = 1)]
    pub fn set_render_type(
        self,
        value: crate::unity_engine::rendering::universal::camerarendertype::CameraRenderType,
    ) -> ();

    #[method(name = "get_cameraStack", args = 0)]
    pub fn get_camera_stack(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::unity_engine::camera::Camera>;

    #[method(name = "UpdateCameraStack", args = 0)]
    pub fn update_camera_stack(self) -> ();

    #[method(name = "get_clearDepth", args = 0)]
    pub fn get_clear_depth(self) -> bool;

    #[method(name = "get_requiresDepthTexture", args = 0)]
    pub fn get_requires_depth_texture(self) -> bool;

    #[method(name = "set_requiresDepthTexture", args = 1)]
    pub fn set_requires_depth_texture(self, value: bool) -> ();

    #[method(name = "get_requiresColorTexture", args = 0)]
    pub fn get_requires_color_texture(self) -> bool;

    #[method(name = "set_requiresColorTexture", args = 1)]
    pub fn set_requires_color_texture(self, value: bool) -> ();

    #[method(name = "get_scriptableRenderer", args = 0)]
    pub fn get_scriptable_renderer(
        self,
    ) -> crate::unity_engine::rendering::universal::scriptablerenderer::ScriptableRenderer;

    #[method(name = "SetRenderer", args = 1)]
    pub fn set_renderer(self, index: i32) -> ();

    #[method(name = "get_volumeLayerMask", args = 0)]
    pub fn get_volume_layer_mask(self) -> crate::unity_engine::layermask::LayerMask;

    #[method(name = "set_volumeLayerMask", args = 1)]
    pub fn set_volume_layer_mask(self, value: crate::unity_engine::layermask::LayerMask) -> ();

    #[method(name = "get_volumeTrigger", args = 0)]
    pub fn get_volume_trigger(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_volumeTrigger", args = 1)]
    pub fn set_volume_trigger(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_renderPostProcessing", args = 0)]
    pub fn get_render_post_processing(self) -> bool;

    #[method(name = "set_renderPostProcessing", args = 1)]
    pub fn set_render_post_processing(self, value: bool) -> ();

    #[method(name = "get_antialiasing", args = 0)]
    pub fn get_antialiasing(
        self,
    ) -> crate::unity_engine::rendering::universal::antialiasingmode::AntialiasingMode;

    #[method(name = "set_antialiasing", args = 1)]
    pub fn set_antialiasing(
        self,
        value: crate::unity_engine::rendering::universal::antialiasingmode::AntialiasingMode,
    ) -> ();

    #[method(name = "get_antialiasingQuality", args = 0)]
    pub fn get_antialiasing_quality(
        self,
    ) -> crate::unity_engine::rendering::universal::antialiasingquality::AntialiasingQuality;

    #[method(name = "set_antialiasingQuality", args = 1)]
    pub fn set_antialiasing_quality(
        self,
        value: crate::unity_engine::rendering::universal::antialiasingquality::AntialiasingQuality,
    ) -> ();

    #[method(name = "get_stopNaN", args = 0)]
    pub fn get_stop_na_n(self) -> bool;

    #[method(name = "set_stopNaN", args = 1)]
    pub fn set_stop_na_n(self, value: bool) -> ();

    #[method(name = "get_dithering", args = 0)]
    pub fn get_dithering(self) -> bool;

    #[method(name = "set_dithering", args = 1)]
    pub fn set_dithering(self, value: bool) -> ();

    #[method(name = "get_allowXRRendering", args = 0)]
    pub fn get_allow_xr_rendering(self) -> bool;

    #[method(name = "set_allowXRRendering", args = 1)]
    pub fn set_allow_xr_rendering(self, value: bool) -> ();

    #[method(name = "get_isAutoReduction", args = 0)]
    pub fn get_is_auto_reduction(self) -> bool;

    #[method(name = "set_isAutoReduction", args = 1)]
    pub fn set_is_auto_reduction(self, value: bool) -> ();

    #[method(name = "get_customFlagRenderingMaster", args = 0)]
    pub fn get_custom_flag_rendering_master(self) -> bool;

    #[method(name = "set_customFlagRenderingMaster", args = 1)]
    pub fn set_custom_flag_rendering_master(self, value: bool) -> ();

    #[method(name = "get_customRenderingFlag", args = 0)]
    pub fn get_custom_rendering_flag (self ,) -> crate :: unity_engine :: rendering :: universal :: customcamerarenderingflag :: CustomCameraRenderingFlag ;

    #[method(name = "set_customRenderingFlag", args = 1)]
    pub fn set_custom_rendering_flag(
        self,
        value : crate :: unity_engine :: rendering :: universal :: customcamerarenderingflag :: CustomCameraRenderingFlag,
    ) -> ();

    #[method(name = "OnBeforeSerialize", args = 0)]
    pub fn on_before_serialize(self) -> ();

    #[method(name = "OnAfterDeserialize", args = 0)]
    pub fn on_after_deserialize(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-universaladditionalcameradata")]
impl UniversalAdditionalCameraData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UniversalAdditionalCameraData),
                ::core::stringify!(new),
            )
        });
        <Self as IUniversalAdditionalCameraDataMethods>::ctor(this);
        this
    }
}
