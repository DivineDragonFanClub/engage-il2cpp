
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/debugloghandler/DebugLogHandler.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "DebugLogHandler")]
#[parent(crate::system::object::Object)]
pub struct DebugLogHandler {}

#[cfg(feature = "unity_engine-debugloghandler")]
#[::unity2::methods]
impl DebugLogHandler {
    #[method(name = "Internal_Log", args = 4)]
    pub fn internal_log(
        level: crate::unity_engine::logtype::LogType,
        options: crate::unity_engine::logoption::LogOption,
        msg: ::unity2::Il2CppString,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "LogFormat", args = 4)]
    pub fn log_format(
        self,
        log_type: crate::unity_engine::logtype::LogType,
        context: crate::unity_engine::object_2::Object_2,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-debugloghandler")]
impl DebugLogHandler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugLogHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugLogHandlerMethods>::ctor(this);
        this
    }
}
