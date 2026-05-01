
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/unitylogwriter/UnityLogWriter.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "UnityLogWriter")]
pub struct UnityLogWriter {}

#[cfg(feature = "unity_engine-unitylogwriter")]
#[::unity2::methods]
impl UnityLogWriter {
    #[method(name = "WriteStringToUnityLog", args = 1)]
    pub fn write_string_to_unity_log(s: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteStringToUnityLogImpl", args = 1)]
    pub fn write_string_to_unity_log_impl(s: ::unity2::Il2CppString) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init() -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, value: u16) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write_2(self, s: ::unity2::Il2CppString) -> ();

    #[method(name = "Write", args = 3)]
    pub fn write_3(self, buffer: ::unity2::Array<u16>, index: i32, count: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-unitylogwriter")]
impl UnityLogWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityLogWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityLogWriterMethods>::ctor(this);
        this
    }
}
