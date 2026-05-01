
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::profiling::sampler::ISampler;
use crate::unity_engine::profiling::sampler::Sampler;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/profiling/customsampler/CustomSampler.md")))]
#[::unity2::class(namespace = "UnityEngine.Profiling", name = "CustomSampler")]
#[parent(crate::unity_engine::profiling::sampler::Sampler)]
pub struct CustomSampler {
    #[static_field]
    #[rename(name = "s_InvalidCustomSampler")]
    pub s_invalid_custom_sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
}

#[cfg(feature = "unity_engine-profiling-customsampler")]
#[::unity2::methods]
impl CustomSampler {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(
        name: ::unity2::Il2CppString,
        collect_gpu_data: bool,
    ) -> crate::unity_engine::profiling::customsampler::CustomSampler;

    #[method(name = "CreateInternal", args = 2)]
    pub fn create_internal(
        name: ::unity2::Il2CppString,
        collect_gpu_data: bool,
    ) -> ::unity2::IntPtr;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-profiling-customsampler")]
impl CustomSampler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomSampler),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomSamplerMethods>::ctor(this);
        this
    }

    pub fn new_2(ptr: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomSampler),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICustomSamplerMethods>::ctor_2(this, ptr);
        this
    }
}
