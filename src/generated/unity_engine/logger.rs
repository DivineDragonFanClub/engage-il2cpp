
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/logger/Logger.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Logger")]
#[parent(crate::system::object::Object)]
pub struct Logger {}

#[cfg(feature = "unity_engine-logger")]
#[::unity2::methods]
impl Logger {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, log_handler: crate::unity_engine::iloghandler::ILogHandler) -> ();

    #[method(name = "get_logHandler", args = 0)]
    pub fn get_log_handler(self) -> crate::unity_engine::iloghandler::ILogHandler;

    #[method(name = "set_logHandler", args = 1)]
    pub fn set_log_handler(self, value: crate::unity_engine::iloghandler::ILogHandler) -> ();

    #[method(name = "get_logEnabled", args = 0)]
    pub fn get_log_enabled(self) -> bool;

    #[method(name = "set_logEnabled", args = 1)]
    pub fn set_log_enabled(self, value: bool) -> ();

    #[method(name = "get_filterLogType", args = 0)]
    pub fn get_filter_log_type(self) -> crate::unity_engine::logtype::LogType;

    #[method(name = "set_filterLogType", args = 1)]
    pub fn set_filter_log_type(self, value: crate::unity_engine::logtype::LogType) -> ();

    #[method(name = "IsLogTypeAllowed", args = 1)]
    pub fn is_log_type_allowed(self, log_type: crate::unity_engine::logtype::LogType) -> bool;

    #[method(name = "GetString", args = 1)]
    pub fn get_string(message: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "Log", args = 2)]
    pub fn log(
        self,
        log_type: crate::unity_engine::logtype::LogType,
        message: crate::system::object::Object,
    ) -> ();

    #[method(name = "Log", args = 3)]
    pub fn log_2(
        self,
        log_type: crate::unity_engine::logtype::LogType,
        message: crate::system::object::Object,
        context: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "LogError", args = 2)]
    pub fn log_error(
        self,
        tag: ::unity2::Il2CppString,
        message: crate::system::object::Object,
    ) -> ();

    #[method(name = "LogFormat", args = 3)]
    pub fn log_format(
        self,
        log_type: crate::unity_engine::logtype::LogType,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "LogFormat", args = 4)]
    pub fn log_format_2(
        self,
        log_type: crate::unity_engine::logtype::LogType,
        context: crate::unity_engine::object_2::Object_2,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();
}

#[cfg(feature = "unity_engine-logger")]
impl Logger {
    pub fn new(log_handler: crate::unity_engine::iloghandler::ILogHandler) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Logger),
                ::core::stringify!(new),
            )
        });
        <Self as ILoggerMethods>::ctor(this, log_handler);
        this
    }
}
