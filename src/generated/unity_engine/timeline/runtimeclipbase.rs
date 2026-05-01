
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::timeline::runtimeelement::IRuntimeElement;
use crate::unity_engine::timeline::runtimeelement::RuntimeElement;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/runtimeclipbase/RuntimeClipBase.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "RuntimeClipBase")]
#[parent(crate::unity_engine::timeline::runtimeelement::RuntimeElement)]
pub struct RuntimeClipBase {}

#[cfg(feature = "unity_engine-timeline-runtimeclipbase")]
#[::unity2::methods]
impl RuntimeClipBase {
    #[method(name = "get_start", args = 0)]
    pub fn get_start(self) -> f64;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f64;

    #[method(name = "get_intervalStart", args = 0)]
    pub fn get_interval_start(self) -> i64;

    #[method(name = "get_intervalEnd", args = 0)]
    pub fn get_interval_end(self) -> i64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-runtimeclipbase")]
impl RuntimeClipBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeClipBase),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeClipBaseMethods>::ctor(this);
        this
    }
}
