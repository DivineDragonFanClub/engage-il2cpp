
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/runtimeelement/RuntimeElement.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "RuntimeElement")]
#[parent(crate::system::object::Object)]
pub struct RuntimeElement {}

#[cfg(feature = "unity_engine-timeline-runtimeelement")]
#[::unity2::methods]
impl RuntimeElement {
    #[method(name = "get_intervalStart", args = 0)]
    pub fn get_interval_start(self) -> i64;

    #[method(name = "get_intervalEnd", args = 0)]
    pub fn get_interval_end(self) -> i64;

    #[method(name = "get_intervalBit", args = 0)]
    pub fn get_interval_bit(self) -> i32;

    #[method(name = "set_intervalBit", args = 1)]
    pub fn set_interval_bit(self, value: i32) -> ();

    #[method(name = "set_enable", args = 1)]
    pub fn set_enable(self, value: bool) -> ();

    #[method(name = "EvaluateAt", args = 2)]
    pub fn evaluate_at(
        self,
        local_time: f64,
        frame_data: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-runtimeelement")]
impl RuntimeElement {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeElement),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeElementMethods>::ctor(this);
        this
    }
}
