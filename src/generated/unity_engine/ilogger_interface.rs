
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ilogger_interface/ILogger_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ILogger")]
pub struct ILogger_Interface {}

#[cfg(feature = "unity_engine-ilogger_interface")]
#[::unity2::methods]
impl ILogger_Interface {
    #[method(name = "get_logHandler", args = 0)]
    pub fn get_log_handler(self) -> crate::unity_engine::iloghandler::ILogHandler;

    #[method(name = "get_logEnabled", args = 0)]
    pub fn get_log_enabled(self) -> bool;

    #[method(name = "set_logEnabled", args = 1)]
    pub fn set_log_enabled(self, value: bool) -> ();

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
}
