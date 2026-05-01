
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/sliderstate/SliderState.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SliderState")]
#[parent(crate::system::object::Object)]
pub struct SliderState {
    #[rename(name = "dragStartPos")]
    pub drag_start_pos: f32,
    #[rename(name = "dragStartValue")]
    pub drag_start_value: f32,
    #[rename(name = "isDragging")]
    pub is_dragging: bool,
}

#[cfg(feature = "unity_engine-sliderstate")]
#[::unity2::methods]
impl SliderState {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-sliderstate")]
impl SliderState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SliderState),
                ::core::stringify!(new),
            )
        });
        <Self as ISliderStateMethods>::ctor(this);
        this
    }
}
