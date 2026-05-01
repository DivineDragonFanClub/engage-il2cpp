
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::timeline::runtimeelement::IRuntimeElement;
use crate::unity_engine::timeline::runtimeelement::RuntimeElement;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/infiniteruntimeclip/InfiniteRuntimeClip.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "InfiniteRuntimeClip")]
#[parent(crate::unity_engine::timeline::runtimeelement::RuntimeElement)]
pub struct InfiniteRuntimeClip {
    #[rename(name = "m_Playable")]
    pub m_playable: crate::unity_engine::playables::playable::Playable,
    #[static_field]
    #[rename(name = "kIntervalEnd")]
    pub k_interval_end: i64,
}

#[cfg(feature = "unity_engine-timeline-infiniteruntimeclip")]
#[::unity2::methods]
impl InfiniteRuntimeClip {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, playable: crate::unity_engine::playables::playable::Playable) -> ();

    #[method(name = "get_intervalStart", args = 0)]
    pub fn get_interval_start(self) -> i64;

    #[method(name = "get_intervalEnd", args = 0)]
    pub fn get_interval_end(self) -> i64;

    #[method(name = "set_enable", args = 1)]
    pub fn set_enable(self, value: bool) -> ();

    #[method(name = "EvaluateAt", args = 2)]
    pub fn evaluate_at(
        self,
        local_time: f64,
        frame_data: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-timeline-infiniteruntimeclip")]
impl InfiniteRuntimeClip {
    pub fn new(playable: crate::unity_engine::playables::playable::Playable) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InfiniteRuntimeClip),
                ::core::stringify!(new),
            )
        });
        <Self as IInfiniteRuntimeClipMethods>::ctor(this, playable);
        this
    }
}
