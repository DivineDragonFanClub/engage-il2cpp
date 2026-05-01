
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/canvas/Canvas_WillRenderCanvases.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Canvas.WillRenderCanvases")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Canvas_WillRenderCanvases {}

#[cfg(feature = "unity_engine-canvas")]
#[::unity2::methods]
impl Canvas_WillRenderCanvases {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-canvas")]
impl Canvas_WillRenderCanvases {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Canvas_WillRenderCanvases),
                ::core::stringify!(new),
            )
        });
        <Self as ICanvas_WillRenderCanvasesMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/canvas/Canvas.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Canvas")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Canvas {
    #[static_field]
    #[rename(name = "preWillRenderCanvases")]
    pub pre_will_render_canvases: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
    #[static_field]
    #[rename(name = "willRenderCanvases")]
    pub will_render_canvases: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
}

#[cfg(feature = "unity_engine-canvas")]
#[::unity2::methods]
impl Canvas {
    #[method(name = "add_preWillRenderCanvases", args = 1)]
    pub fn add_pre_will_render_canvases(
        value: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
    ) -> ();

    #[method(name = "remove_preWillRenderCanvases", args = 1)]
    pub fn remove_pre_will_render_canvases(
        value: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
    ) -> ();

    #[method(name = "add_willRenderCanvases", args = 1)]
    pub fn add_will_render_canvases(
        value: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
    ) -> ();

    #[method(name = "remove_willRenderCanvases", args = 1)]
    pub fn remove_will_render_canvases(
        value: crate::unity_engine::canvas::Canvas_WillRenderCanvases,
    ) -> ();

    #[method(name = "get_renderMode", args = 0)]
    pub fn get_render_mode(self) -> crate::unity_engine::rendermode::RenderMode;

    #[method(name = "set_renderMode", args = 1)]
    pub fn set_render_mode(self, value: crate::unity_engine::rendermode::RenderMode) -> ();

    #[method(name = "get_isRootCanvas", args = 0)]
    pub fn get_is_root_canvas(self) -> bool;

    #[method(name = "get_pixelRect", args = 0)]
    pub fn get_pixel_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "get_scaleFactor", args = 0)]
    pub fn get_scale_factor(self) -> f32;

    #[method(name = "set_scaleFactor", args = 1)]
    pub fn set_scale_factor(self, value: f32) -> ();

    #[method(name = "get_referencePixelsPerUnit", args = 0)]
    pub fn get_reference_pixels_per_unit(self) -> f32;

    #[method(name = "set_referencePixelsPerUnit", args = 1)]
    pub fn set_reference_pixels_per_unit(self, value: f32) -> ();

    #[method(name = "get_overridePixelPerfect", args = 0)]
    pub fn get_override_pixel_perfect(self) -> bool;

    #[method(name = "set_overridePixelPerfect", args = 1)]
    pub fn set_override_pixel_perfect(self, value: bool) -> ();

    #[method(name = "get_pixelPerfect", args = 0)]
    pub fn get_pixel_perfect(self) -> bool;

    #[method(name = "set_pixelPerfect", args = 1)]
    pub fn set_pixel_perfect(self, value: bool) -> ();

    #[method(name = "get_planeDistance", args = 0)]
    pub fn get_plane_distance(self) -> f32;

    #[method(name = "set_planeDistance", args = 1)]
    pub fn set_plane_distance(self, value: f32) -> ();

    #[method(name = "get_renderOrder", args = 0)]
    pub fn get_render_order(self) -> i32;

    #[method(name = "get_overrideSorting", args = 0)]
    pub fn get_override_sorting(self) -> bool;

    #[method(name = "set_overrideSorting", args = 1)]
    pub fn set_override_sorting(self, value: bool) -> ();

    #[method(name = "get_sortingOrder", args = 0)]
    pub fn get_sorting_order(self) -> i32;

    #[method(name = "set_sortingOrder", args = 1)]
    pub fn set_sorting_order(self, value: i32) -> ();

    #[method(name = "get_targetDisplay", args = 0)]
    pub fn get_target_display(self) -> i32;

    #[method(name = "set_targetDisplay", args = 1)]
    pub fn set_target_display(self, value: i32) -> ();

    #[method(name = "get_sortingLayerID", args = 0)]
    pub fn get_sorting_layer_id(self) -> i32;

    #[method(name = "set_sortingLayerID", args = 1)]
    pub fn set_sorting_layer_id(self, value: i32) -> ();

    #[method(name = "get_cachedSortingLayerValue", args = 0)]
    pub fn get_cached_sorting_layer_value(self) -> i32;

    #[method(name = "get_additionalShaderChannels", args = 0)]
    pub fn get_additional_shader_channels(
        self,
    ) -> crate::unity_engine::additionalcanvasshaderchannels::AdditionalCanvasShaderChannels;

    #[method(name = "set_additionalShaderChannels", args = 1)]
    pub fn set_additional_shader_channels(
        self,
        value: crate::unity_engine::additionalcanvasshaderchannels::AdditionalCanvasShaderChannels,
    ) -> ();

    #[method(name = "get_sortingLayerName", args = 0)]
    pub fn get_sorting_layer_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_sortingLayerName", args = 1)]
    pub fn set_sorting_layer_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_rootCanvas", args = 0)]
    pub fn get_root_canvas(self) -> crate::unity_engine::canvas::Canvas;

    #[method(name = "get_renderingDisplaySize", args = 0)]
    pub fn get_rendering_display_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_worldCamera", args = 0)]
    pub fn get_world_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_worldCamera", args = 1)]
    pub fn set_world_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_normalizedSortingGridSize", args = 0)]
    pub fn get_normalized_sorting_grid_size(self) -> f32;

    #[method(name = "set_normalizedSortingGridSize", args = 1)]
    pub fn set_normalized_sorting_grid_size(self, value: f32) -> ();

    #[method(name = "get_sortingGridNormalizedSize", args = 0)]
    pub fn get_sorting_grid_normalized_size(self) -> i32;

    #[method(name = "set_sortingGridNormalizedSize", args = 1)]
    pub fn set_sorting_grid_normalized_size(self, value: i32) -> ();

    #[method(name = "GetDefaultCanvasTextMaterial", args = 0)]
    pub fn get_default_canvas_text_material() -> crate::unity_engine::material::Material;

    #[method(name = "GetDefaultCanvasMaterial", args = 0)]
    pub fn get_default_canvas_material() -> crate::unity_engine::material::Material;

    #[method(name = "GetETC1SupportedCanvasMaterial", args = 0)]
    pub fn get_etc1_supported_canvas_material() -> crate::unity_engine::material::Material;

    #[method(name = "UpdateCanvasRectTransform", args = 1)]
    pub fn update_canvas_rect_transform(self, align_with_camera: bool) -> ();

    #[method(name = "ForceUpdateCanvases", args = 0)]
    pub fn force_update_canvases() -> ();

    #[method(name = "SendPreWillRenderCanvases", args = 0)]
    pub fn send_pre_will_render_canvases() -> ();

    #[method(name = "SendWillRenderCanvases", args = 0)]
    pub fn send_will_render_canvases() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_pixelRect_Injected", args = 1)]
    pub fn get_pixel_rect_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_renderingDisplaySize_Injected", args = 1)]
    pub fn get_rendering_display_size_injected(
        self,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();
}

#[cfg(feature = "unity_engine-canvas")]
impl Canvas {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Canvas),
                ::core::stringify!(new),
            )
        });
        <Self as ICanvasMethods>::ctor(this);
        this
    }
}
