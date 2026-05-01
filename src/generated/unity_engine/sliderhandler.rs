
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/sliderhandler/SliderHandler.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SliderHandler {
    pub position: crate::unity_engine::rect::Rect,
    pub current_value: f32,
    pub size: f32,
    pub start: f32,
    pub end: f32,
    pub slider: crate::unity_engine::guistyle::GUIStyle,
    pub thumb: crate::unity_engine::guistyle::GUIStyle,
    pub thumb_extent: crate::unity_engine::guistyle::GUIStyle,
    pub horiz: bool,
    pub id: i32,
}

impl ::unity2::ClassIdentity for SliderHandler {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "SliderHandler";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SliderHandler {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-sliderhandler")]
#[::unity2::methods(value)]
impl SliderHandler {
    #[method(name = ".ctor", args = 10)]
    pub fn ctor(
        self,
        position: crate::unity_engine::rect::Rect,
        current_value: f32,
        size: f32,
        start: f32,
        end: f32,
        slider: crate::unity_engine::guistyle::GUIStyle,
        thumb: crate::unity_engine::guistyle::GUIStyle,
        horiz: bool,
        id: i32,
        thumb_extent: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "Handle", args = 0)]
    pub fn handle(self) -> f32;

    #[method(name = "OnMouseDown", args = 0)]
    pub fn on_mouse_down(self) -> f32;

    #[method(name = "OnMouseDrag", args = 0)]
    pub fn on_mouse_drag(self) -> f32;

    #[method(name = "OnMouseUp", args = 0)]
    pub fn on_mouse_up(self) -> f32;

    #[method(name = "OnRepaint", args = 0)]
    pub fn on_repaint(self) -> f32;

    #[method(name = "CurrentEventType", args = 0)]
    pub fn current_event_type(self) -> crate::unity_engine::eventtype::EventType;

    #[method(name = "CurrentScrollTroughSide", args = 0)]
    pub fn current_scroll_trough_side(self) -> i32;

    #[method(name = "IsEmptySlider", args = 0)]
    pub fn is_empty_slider(self) -> bool;

    #[method(name = "SupportsPageMovements", args = 0)]
    pub fn supports_page_movements(self) -> bool;

    #[method(name = "PageMovementValue", args = 0)]
    pub fn page_movement_value(self) -> f32;

    #[method(name = "PageUpMovementBound", args = 0)]
    pub fn page_up_movement_bound(self) -> f32;

    #[method(name = "CurrentEvent", args = 0)]
    pub fn current_event(self) -> crate::unity_engine::event::Event;

    #[method(name = "ValueForCurrentMousePosition", args = 0)]
    pub fn value_for_current_mouse_position(self) -> f32;

    #[method(name = "Clamp", args = 1)]
    pub fn clamp(self, value: f32) -> f32;

    #[method(name = "ThumbSelectionRect", args = 0)]
    pub fn thumb_selection_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "StartDraggingWithValue", args = 1)]
    pub fn start_dragging_with_value(self, drag_start_value: f32) -> ();

    #[method(name = "SliderState", args = 0)]
    pub fn slider_state(self) -> crate::unity_engine::sliderstate::SliderState;

    #[method(name = "ThumbExtRect", args = 0)]
    pub fn thumb_ext_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "ThumbRect", args = 0)]
    pub fn thumb_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "VerticalThumbRect", args = 0)]
    pub fn vertical_thumb_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "HorizontalThumbRect", args = 0)]
    pub fn horizontal_thumb_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "ClampedCurrentValue", args = 0)]
    pub fn clamped_current_value(self) -> f32;

    #[method(name = "MousePosition", args = 0)]
    pub fn mouse_position(self) -> f32;

    #[method(name = "ValuesPerPixel", args = 0)]
    pub fn values_per_pixel(self) -> f32;

    #[method(name = "ThumbSize", args = 0)]
    pub fn thumb_size(self) -> f32;

    #[method(name = "MaxValue", args = 0)]
    pub fn max_value(self) -> f32;

    #[method(name = "MinValue", args = 0)]
    pub fn min_value(self) -> f32;
}
