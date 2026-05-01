
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/profiling/profiler/Profiler.md")))]
#[::unity2::class(namespace = "UnityEngine.Profiling", name = "Profiler")]
#[parent(crate::system::object::Object)]
pub struct Profiler {}

#[cfg(feature = "unity_engine-profiling-profiler")]
#[::unity2::methods]
impl Profiler {
    #[method(name = "GetRuntimeMemorySizeLong", args = 1)]
    pub fn get_runtime_memory_size_long(o: crate::unity_engine::object_2::Object_2) -> i64;

    #[method(name = "GetMonoUsedSizeLong", args = 0)]
    pub fn get_mono_used_size_long() -> i64;
}
