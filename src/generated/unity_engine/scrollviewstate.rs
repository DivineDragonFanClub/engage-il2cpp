
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scrollviewstate/ScrollViewState.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ScrollViewState")]
#[parent(crate::system::object::Object)]
pub struct ScrollViewState {
    #[rename(name = "position")]
    pub position: crate::unity_engine::rect::Rect,
    #[rename(name = "visibleRect")]
    pub visible_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "viewRect")]
    pub view_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "scrollPosition")]
    pub scroll_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "apply")]
    pub apply: bool,
    #[rename(name = "isDuringTouchScroll")]
    pub is_during_touch_scroll: bool,
    #[rename(name = "touchScrollStartMousePosition")]
    pub touch_scroll_start_mouse_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "touchScrollStartPosition")]
    pub touch_scroll_start_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "velocity")]
    pub velocity: crate::unity_engine::vector2::Vector2,
    #[rename(name = "previousTimeSinceStartup")]
    pub previous_time_since_startup: f32,
}

#[cfg(feature = "unity_engine-scrollviewstate")]
#[::unity2::methods]
impl ScrollViewState {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-scrollviewstate")]
impl ScrollViewState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScrollViewState),
                ::core::stringify!(new),
            )
        });
        <Self as IScrollViewStateMethods>::ctor(this);
        this
    }
}
