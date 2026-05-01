
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/profiling/sampler/Sampler.md")))]
#[::unity2::class(namespace = "UnityEngine.Profiling", name = "Sampler")]
#[parent(crate::system::object::Object)]
pub struct Sampler {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_InvalidSampler")]
    pub s_invalid_sampler: crate::unity_engine::profiling::sampler::Sampler,
}

#[cfg(feature = "unity_engine-profiling-sampler")]
#[::unity2::methods]
impl Sampler {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_isValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "GetRecorder", args = 0)]
    pub fn get_recorder(self) -> crate::unity_engine::profiling::recorder::Recorder;

    #[method(name = "GetRecorderInternal", args = 1)]
    pub fn get_recorder_internal(ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-profiling-sampler")]
impl Sampler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Sampler),
                ::core::stringify!(new),
            )
        });
        <Self as ISamplerMethods>::ctor(this);
        this
    }
}
