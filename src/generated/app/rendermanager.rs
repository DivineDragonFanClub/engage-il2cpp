
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rendermanager/RenderManager.md")))]
#[::unity2::class(namespace = "App", name = "RenderManager")]
#[parent(crate::system::object::Object)]
pub struct RenderManager {
    #[static_field]
    #[rename(name = "s_ScaleStack")]
    pub s_scale_stack: crate::system::collections::generic::stack_1::Stack_1<f32>,
    #[static_field]
    #[rename(name = "s_ScaleStackOnGpuSaved")]
    pub s_scale_stack_on_gpu_saved: crate::system::collections::generic::stack_1::Stack_1<f32>,
    #[static_field]
    #[rename(name = "s_LodBiasStack")]
    pub s_lod_bias_stack: crate::system::collections::generic::stack_1::Stack_1<f32>,
    #[static_field]
    #[rename(name = "s_LodBiasStackOnGpuSaved")]
    pub s_lod_bias_stack_on_gpu_saved: crate::system::collections::generic::stack_1::Stack_1<f32>,
    #[static_field]
    #[rename(name = "s_BlurStack")]
    pub s_blur_stack: crate::system::collections::generic::stack_1::Stack_1<i32>,
    #[static_field]
    #[rename(name = "s_ColorStack")]
    pub s_color_stack:
        crate::system::collections::generic::stack_1::Stack_1<crate::unity_engine::color::Color>,
    #[static_field]
    #[rename(name = "s_ColorRateStack")]
    pub s_color_rate_stack: crate::system::collections::generic::stack_1::Stack_1<f32>,
    #[static_field]
    #[rename(name = "s_CrossFadeAnimationDuration")]
    pub s_cross_fade_animation_duration: crate::system::collections::generic::stack_1::Stack_1<f32>,
}

#[cfg(feature = "app-rendermanager")]
#[::unity2::methods]
impl RenderManager {
    #[method(name = "PushRenderScale", args = 1)]
    pub fn push_render_scale(name: ::unity2::Il2CppString) -> ();

    #[method(name = "PushRenderScale", args = 1)]
    pub fn push_render_scale_2(scale: f32) -> ();

    #[method(name = "PushRenderScale", args = 2)]
    pub fn push_render_scale_3(
        name: ::unity2::Il2CppString,
        name_on_gpu_save: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PushRenderScale", args = 2)]
    pub fn push_render_scale_4(scale: f32, scale_on_gpu_save: f32) -> ();

    #[method(name = "PopRenderScale", args = 0)]
    pub fn pop_render_scale() -> ();

    #[method(name = "PushLodBias", args = 1)]
    pub fn push_lod_bias(lod_bias: f32) -> ();

    #[method(name = "PushLodBias", args = 2)]
    pub fn push_lod_bias_2(lod_bias: f32, lod_bias_on_gpu_saved: f32) -> ();

    #[method(name = "PopLodBias", args = 0)]
    pub fn pop_lod_bias() -> ();

    #[method(name = "PushCrossFadeAnimationDuration", args = 1)]
    pub fn push_cross_fade_animation_duration(duration: f32) -> ();

    #[method(name = "PopCrossFadeAnimationDuration", args = 0)]
    pub fn pop_cross_fade_animation_duration() -> ();

    #[method(name = "PushCustomBlur", args = 1)]
    pub fn push_custom_blur(name: ::unity2::Il2CppString) -> ();

    #[method(name = "PushCustomBlur", args = 1)]
    pub fn push_custom_blur_2(blur: i32) -> ();

    #[method(name = "PopCustomBlur", args = 0)]
    pub fn pop_custom_blur() -> ();

    #[method(name = "PushColorRate", args = 1)]
    pub fn push_color_rate(name: ::unity2::Il2CppString) -> ();

    #[method(name = "PushColorRate", args = 1)]
    pub fn push_color_rate_2(rate: f32) -> ();

    #[method(name = "PopColorRate", args = 0)]
    pub fn pop_color_rate() -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "UpdateGpuPerformanceMode", args = 0)]
    pub fn update_gpu_performance_mode() -> ();

    #[method(name = "GetFloat", args = 2)]
    pub fn get_float(name: ::unity2::Il2CppString, def: f32) -> f32;

    #[method(name = "GetInt", args = 2)]
    pub fn get_int(name: ::unity2::Il2CppString, def: i32) -> i32;

    #[method(name = "CommitScale", args = 0)]
    pub fn commit_scale() -> ();

    #[method(name = "CommitLodBias", args = 0)]
    pub fn commit_lod_bias() -> ();

    #[method(name = "CommitBlur", args = 0)]
    pub fn commit_blur() -> ();

    #[method(name = "CommitColor", args = 0)]
    pub fn commit_color() -> ();

    #[method(name = "CommitColorRate", args = 0)]
    pub fn commit_color_rate() -> ();

    #[method(name = "CommitCrossFadeAnimationDuration", args = 0)]
    pub fn commit_cross_fade_animation_duration() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-rendermanager")]
impl RenderManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderManager),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderManagerMethods>::ctor(this);
        this
    }
}
