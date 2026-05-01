
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/light/Light.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Light")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Light {
    #[rename(name = "m_BakedIndex")]
    pub m_baked_index: i32,
}

#[cfg(feature = "unity_engine-light")]
#[::unity2::methods]
impl Light {
    #[method(name = "get_type", args = 0)]
    pub fn get_type(self) -> crate::unity_engine::lighttype::LightType;

    #[method(name = "set_type", args = 1)]
    pub fn set_type(self, value: crate::unity_engine::lighttype::LightType) -> ();

    #[method(name = "get_shape", args = 0)]
    pub fn get_shape(self) -> crate::unity_engine::lightshape::LightShape;

    #[method(name = "set_shape", args = 1)]
    pub fn set_shape(self, value: crate::unity_engine::lightshape::LightShape) -> ();

    #[method(name = "get_spotAngle", args = 0)]
    pub fn get_spot_angle(self) -> f32;

    #[method(name = "set_spotAngle", args = 1)]
    pub fn set_spot_angle(self, value: f32) -> ();

    #[method(name = "get_innerSpotAngle", args = 0)]
    pub fn get_inner_spot_angle(self) -> f32;

    #[method(name = "set_innerSpotAngle", args = 1)]
    pub fn set_inner_spot_angle(self, value: f32) -> ();

    #[method(name = "get_color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_colorTemperature", args = 0)]
    pub fn get_color_temperature(self) -> f32;

    #[method(name = "set_colorTemperature", args = 1)]
    pub fn set_color_temperature(self, value: f32) -> ();

    #[method(name = "get_useColorTemperature", args = 0)]
    pub fn get_use_color_temperature(self) -> bool;

    #[method(name = "set_useColorTemperature", args = 1)]
    pub fn set_use_color_temperature(self, value: bool) -> ();

    #[method(name = "get_intensity", args = 0)]
    pub fn get_intensity(self) -> f32;

    #[method(name = "set_intensity", args = 1)]
    pub fn set_intensity(self, value: f32) -> ();

    #[method(name = "get_bounceIntensity", args = 0)]
    pub fn get_bounce_intensity(self) -> f32;

    #[method(name = "set_bounceIntensity", args = 1)]
    pub fn set_bounce_intensity(self, value: f32) -> ();

    #[method(name = "get_useBoundingSphereOverride", args = 0)]
    pub fn get_use_bounding_sphere_override(self) -> bool;

    #[method(name = "set_useBoundingSphereOverride", args = 1)]
    pub fn set_use_bounding_sphere_override(self, value: bool) -> ();

    #[method(name = "get_boundingSphereOverride", args = 0)]
    pub fn get_bounding_sphere_override(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_boundingSphereOverride", args = 1)]
    pub fn set_bounding_sphere_override(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "get_useViewFrustumForShadowCasterCull", args = 0)]
    pub fn get_use_view_frustum_for_shadow_caster_cull(self) -> bool;

    #[method(name = "set_useViewFrustumForShadowCasterCull", args = 1)]
    pub fn set_use_view_frustum_for_shadow_caster_cull(self, value: bool) -> ();

    #[method(name = "get_shadowCustomResolution", args = 0)]
    pub fn get_shadow_custom_resolution(self) -> i32;

    #[method(name = "set_shadowCustomResolution", args = 1)]
    pub fn set_shadow_custom_resolution(self, value: i32) -> ();

    #[method(name = "get_shadowBias", args = 0)]
    pub fn get_shadow_bias(self) -> f32;

    #[method(name = "set_shadowBias", args = 1)]
    pub fn set_shadow_bias(self, value: f32) -> ();

    #[method(name = "get_shadowNormalBias", args = 0)]
    pub fn get_shadow_normal_bias(self) -> f32;

    #[method(name = "set_shadowNormalBias", args = 1)]
    pub fn set_shadow_normal_bias(self, value: f32) -> ();

    #[method(name = "get_shadowNearPlane", args = 0)]
    pub fn get_shadow_near_plane(self) -> f32;

    #[method(name = "set_shadowNearPlane", args = 1)]
    pub fn set_shadow_near_plane(self, value: f32) -> ();

    #[method(name = "get_useShadowMatrixOverride", args = 0)]
    pub fn get_use_shadow_matrix_override(self) -> bool;

    #[method(name = "set_useShadowMatrixOverride", args = 1)]
    pub fn set_use_shadow_matrix_override(self, value: bool) -> ();

    #[method(name = "get_shadowMatrixOverride", args = 0)]
    pub fn get_shadow_matrix_override(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_shadowMatrixOverride", args = 1)]
    pub fn set_shadow_matrix_override(self, value: crate::unity_engine::matrix4x4::Matrix4x4)
        -> ();

    #[method(name = "get_range", args = 0)]
    pub fn get_range(self) -> f32;

    #[method(name = "set_range", args = 1)]
    pub fn set_range(self, value: f32) -> ();

    #[method(name = "get_flare", args = 0)]
    pub fn get_flare(self) -> crate::unity_engine::flare::Flare;

    #[method(name = "set_flare", args = 1)]
    pub fn set_flare(self, value: crate::unity_engine::flare::Flare) -> ();

    #[method(name = "get_bakingOutput", args = 0)]
    pub fn get_baking_output(self) -> crate::unity_engine::lightbakingoutput::LightBakingOutput;

    #[method(name = "set_bakingOutput", args = 1)]
    pub fn set_baking_output(
        self,
        value: crate::unity_engine::lightbakingoutput::LightBakingOutput,
    ) -> ();

    #[method(name = "get_cullingMask", args = 0)]
    pub fn get_culling_mask(self) -> i32;

    #[method(name = "set_cullingMask", args = 1)]
    pub fn set_culling_mask(self, value: i32) -> ();

    #[method(name = "get_renderingLayerMask", args = 0)]
    pub fn get_rendering_layer_mask(self) -> i32;

    #[method(name = "set_renderingLayerMask", args = 1)]
    pub fn set_rendering_layer_mask(self, value: i32) -> ();

    #[method(name = "get_lightShadowCasterMode", args = 0)]
    pub fn get_light_shadow_caster_mode(
        self,
    ) -> crate::unity_engine::lightshadowcastermode::LightShadowCasterMode;

    #[method(name = "set_lightShadowCasterMode", args = 1)]
    pub fn set_light_shadow_caster_mode(
        self,
        value: crate::unity_engine::lightshadowcastermode::LightShadowCasterMode,
    ) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_shadows", args = 0)]
    pub fn get_shadows(self) -> crate::unity_engine::lightshadows::LightShadows;

    #[method(name = "set_shadows", args = 1)]
    pub fn set_shadows(self, value: crate::unity_engine::lightshadows::LightShadows) -> ();

    #[method(name = "get_shadowStrength", args = 0)]
    pub fn get_shadow_strength(self) -> f32;

    #[method(name = "set_shadowStrength", args = 1)]
    pub fn set_shadow_strength(self, value: f32) -> ();

    #[method(name = "get_shadowResolution", args = 0)]
    pub fn get_shadow_resolution(
        self,
    ) -> crate::unity_engine::rendering::lightshadowresolution::LightShadowResolution;

    #[method(name = "set_shadowResolution", args = 1)]
    pub fn set_shadow_resolution(
        self,
        value: crate::unity_engine::rendering::lightshadowresolution::LightShadowResolution,
    ) -> ();

    #[method(name = "get_shadowSoftness", args = 0)]
    pub fn get_shadow_softness(self) -> f32;

    #[method(name = "set_shadowSoftness", args = 1)]
    pub fn set_shadow_softness(self, value: f32) -> ();

    #[method(name = "get_shadowSoftnessFade", args = 0)]
    pub fn get_shadow_softness_fade(self) -> f32;

    #[method(name = "set_shadowSoftnessFade", args = 1)]
    pub fn set_shadow_softness_fade(self, value: f32) -> ();

    #[method(name = "get_layerShadowCullDistances", args = 0)]
    pub fn get_layer_shadow_cull_distances(self) -> ::unity2::Array<f32>;

    #[method(name = "set_layerShadowCullDistances", args = 1)]
    pub fn set_layer_shadow_cull_distances(self, value: ::unity2::Array<f32>) -> ();

    #[method(name = "get_cookieSize", args = 0)]
    pub fn get_cookie_size(self) -> f32;

    #[method(name = "set_cookieSize", args = 1)]
    pub fn set_cookie_size(self, value: f32) -> ();

    #[method(name = "get_cookie", args = 0)]
    pub fn get_cookie(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "set_cookie", args = 1)]
    pub fn set_cookie(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_renderMode", args = 0)]
    pub fn get_render_mode(self) -> crate::unity_engine::lightrendermode::LightRenderMode;

    #[method(name = "set_renderMode", args = 1)]
    pub fn set_render_mode(
        self,
        value: crate::unity_engine::lightrendermode::LightRenderMode,
    ) -> ();

    #[method(name = "get_bakedIndex", args = 0)]
    pub fn get_baked_index(self) -> i32;

    #[method(name = "set_bakedIndex", args = 1)]
    pub fn set_baked_index(self, value: i32) -> ();

    #[method(name = "AddCommandBuffer", args = 2)]
    pub fn add_command_buffer(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "AddCommandBuffer", args = 3)]
    pub fn add_command_buffer_2(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        shadow_pass_mask: crate::unity_engine::rendering::shadowmappass::ShadowMapPass,
    ) -> ();

    #[method(name = "AddCommandBufferAsync", args = 3)]
    pub fn add_command_buffer_async(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "AddCommandBufferAsync", args = 4)]
    pub fn add_command_buffer_async_2(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        shadow_pass_mask: crate::unity_engine::rendering::shadowmappass::ShadowMapPass,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "RemoveCommandBuffer", args = 2)]
    pub fn remove_command_buffer(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "RemoveCommandBuffers", args = 1)]
    pub fn remove_command_buffers(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
    ) -> ();

    #[method(name = "RemoveAllCommandBuffers", args = 0)]
    pub fn remove_all_command_buffers(self) -> ();

    #[method(name = "GetCommandBuffers", args = 1)]
    pub fn get_command_buffers(
        self,
        evt: crate::unity_engine::rendering::lightevent::LightEvent,
    ) -> ::unity2::Array<crate::unity_engine::rendering::commandbuffer::CommandBuffer>;

    #[method(name = "get_commandBufferCount", args = 0)]
    pub fn get_command_buffer_count(self) -> i32;

    #[method(name = "get_pixelLightCount", args = 0)]
    pub fn get_pixel_light_count() -> i32;

    #[method(name = "set_pixelLightCount", args = 1)]
    pub fn set_pixel_light_count(value: i32) -> ();

    #[method(name = "GetLights", args = 2)]
    pub fn get_lights(
        r#type: crate::unity_engine::lighttype::LightType,
        layer: i32,
    ) -> ::unity2::Array<crate::unity_engine::light::Light>;

    #[method(name = "get_shadowConstantBias", args = 0)]
    pub fn get_shadow_constant_bias(self) -> f32;

    #[method(name = "set_shadowConstantBias", args = 1)]
    pub fn set_shadow_constant_bias(self, value: f32) -> ();

    #[method(name = "get_shadowObjectSizeBias", args = 0)]
    pub fn get_shadow_object_size_bias(self) -> f32;

    #[method(name = "set_shadowObjectSizeBias", args = 1)]
    pub fn set_shadow_object_size_bias(self, value: f32) -> ();

    #[method(name = "get_attenuate", args = 0)]
    pub fn get_attenuate(self) -> bool;

    #[method(name = "set_attenuate", args = 1)]
    pub fn set_attenuate(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_color_Injected", args = 1)]
    pub fn get_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_color_Injected", args = 1)]
    pub fn set_color_injected(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_boundingSphereOverride_Injected", args = 1)]
    pub fn get_bounding_sphere_override_injected(
        self,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "set_boundingSphereOverride_Injected", args = 1)]
    pub fn set_bounding_sphere_override_injected(
        self,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "get_shadowMatrixOverride_Injected", args = 1)]
    pub fn get_shadow_matrix_override_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "set_shadowMatrixOverride_Injected", args = 1)]
    pub fn set_shadow_matrix_override_injected(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_bakingOutput_Injected", args = 1)]
    pub fn get_baking_output_injected(
        self,
        ret: crate::unity_engine::lightbakingoutput::LightBakingOutput,
    ) -> ();

    #[method(name = "set_bakingOutput_Injected", args = 1)]
    pub fn set_baking_output_injected(
        self,
        value: crate::unity_engine::lightbakingoutput::LightBakingOutput,
    ) -> ();
}

#[cfg(feature = "unity_engine-light")]
impl Light {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Light),
                ::core::stringify!(new),
            )
        });
        <Self as ILightMethods>::ctor(this);
        this
    }
}
