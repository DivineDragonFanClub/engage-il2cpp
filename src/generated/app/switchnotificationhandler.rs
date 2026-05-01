
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/switchnotificationhandler/SwitchNotificationHandler.md")))]
#[::unity2::class(namespace = "App", name = "SwitchNotificationHandler")]
#[parent(crate::system::object::Object)]
pub struct SwitchNotificationHandler {
    #[static_field]
    #[rename(name = "OperationModeInitialized")]
    pub operation_mode_initialized: bool,
    #[static_field]
    #[rename(name = "PerformanceModeInitialized")]
    pub performance_mode_initialized: bool,
}

#[cfg(feature = "app-switchnotificationhandler")]
#[::unity2::methods]
impl SwitchNotificationHandler {
    #[method(name = "get_IsDocked", args = 0)]
    pub fn get_is_docked() -> bool;

    #[method(name = "get_IsBoosted", args = 0)]
    pub fn get_is_boosted() -> bool;

    #[method(name = "get_OperationMode", args = 0)]
    pub fn get_operation_mode() -> crate::unity_engine::switch::operation::Operation_OperationMode;

    #[method(name = "set_OperationMode", args = 1)]
    pub fn set_operation_mode(
        value: crate::unity_engine::switch::operation::Operation_OperationMode,
    ) -> ();

    #[method(name = "get_PerformanceMode", args = 0)]
    pub fn get_performance_mode(
    ) -> crate::unity_engine::switch::performance::Performance_PerformanceMode;

    #[method(name = "set_PerformanceMode", args = 1)]
    pub fn set_performance_mode(
        value: crate::unity_engine::switch::performance::Performance_PerformanceMode,
    ) -> ();

    #[method(name = "OnRuntimeMethodLoad", args = 0)]
    pub fn on_runtime_method_load() -> ();

    #[method(name = "OnNotificationMessage", args = 1)]
    pub fn on_notification_message(
        message: crate::unity_engine::switch::notification::Notification_Message,
    ) -> ();

    #[method(name = "UpdateOperationMode", args = 0)]
    pub fn update_operation_mode() -> ();

    #[method(name = "UpdatePerformanceMode", args = 0)]
    pub fn update_performance_mode() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
