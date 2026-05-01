
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/profilingsampler/ProfilingSampler.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ProfilingSampler")]
#[parent(crate::system::object::Object)]
pub struct ProfilingSampler {
    #[rename(name = "m_Recorder")]
    pub m_recorder: crate::unity_engine::profiling::recorder::Recorder,
    #[rename(name = "m_InlineRecorder")]
    pub m_inline_recorder: crate::unity_engine::profiling::recorder::Recorder,
}

#[cfg(feature = "unity_engine-rendering-profilingsampler")]
#[::unity2::methods]
impl ProfilingSampler {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Begin", args = 1)]
    pub fn begin(self, cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer) -> ();

    #[method(name = "End", args = 1)]
    pub fn end(self, cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "get_sampler", args = 0)]
    pub fn get_sampler(self) -> crate::unity_engine::profiling::customsampler::CustomSampler;

    #[method(name = "set_sampler", args = 1)]
    pub fn set_sampler(
        self,
        value: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "get_inlineSampler", args = 0)]
    pub fn get_inline_sampler(self)
        -> crate::unity_engine::profiling::customsampler::CustomSampler;

    #[method(name = "set_inlineSampler", args = 1)]
    pub fn set_inline_sampler(
        self,
        value: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_enableRecording", args = 1)]
    pub fn set_enable_recording(self, value: bool) -> ();

    #[method(name = "get_gpuElapsedTime", args = 0)]
    pub fn get_gpu_elapsed_time(self) -> f32;

    #[method(name = "get_gpuSampleCount", args = 0)]
    pub fn get_gpu_sample_count(self) -> i32;

    #[method(name = "get_cpuElapsedTime", args = 0)]
    pub fn get_cpu_elapsed_time(self) -> f32;

    #[method(name = "get_cpuSampleCount", args = 0)]
    pub fn get_cpu_sample_count(self) -> i32;

    #[method(name = "get_inlineCpuElapsedTime", args = 0)]
    pub fn get_inline_cpu_elapsed_time(self) -> f32;

    #[method(name = "get_inlineCpuSampleCount", args = 0)]
    pub fn get_inline_cpu_sample_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-profilingsampler")]
impl ProfilingSampler {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfilingSampler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfilingSamplerMethods>::ctor(this, name);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfilingSampler),
                ::core::stringify!(new_2),
            )
        });
        <Self as IProfilingSamplerMethods>::ctor_2(this);
        this
    }
}
