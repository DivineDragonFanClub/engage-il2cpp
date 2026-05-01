
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/application/Application_LogCallback.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Application.LogCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Application_LogCallback {}

#[cfg(feature = "unity_engine-application")]
#[::unity2::methods]
impl Application_LogCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        condition: ::unity2::Il2CppString,
        stack_trace: ::unity2::Il2CppString,
        r#type: crate::unity_engine::logtype::LogType,
    ) -> ();
}

#[cfg(feature = "unity_engine-application")]
impl Application_LogCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Application_LogCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IApplication_LogCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/application/Application_LowMemoryCallback.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Application.LowMemoryCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Application_LowMemoryCallback {}

#[cfg(feature = "unity_engine-application")]
#[::unity2::methods]
impl Application_LowMemoryCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-application")]
impl Application_LowMemoryCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Application_LowMemoryCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IApplication_LowMemoryCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/application/Application.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Application")]
#[parent(crate::system::object::Object)]
pub struct Application {
    #[static_field]
    #[rename(name = "lowMemory")]
    pub low_memory: crate::unity_engine::application::Application_LowMemoryCallback,
    #[static_field]
    #[rename(name = "s_LogCallbackHandler")]
    pub s_log_callback_handler: crate::unity_engine::application::Application_LogCallback,
    #[static_field]
    #[rename(name = "s_LogCallbackHandlerThreaded")]
    pub s_log_callback_handler_threaded: crate::unity_engine::application::Application_LogCallback,
    #[static_field]
    #[rename(name = "focusChanged")]
    pub focus_changed: crate::system::action_1::Action_1<bool>,
    #[static_field]
    #[rename(name = "deepLinkActivated")]
    pub deep_link_activated: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "wantsToQuit")]
    pub wants_to_quit: crate::system::func_1::Func_1<bool>,
    #[static_field]
    #[rename(name = "quitting")]
    pub quitting: crate::system::action::Action,
    #[static_field]
    #[rename(name = "unloading")]
    pub unloading: crate::system::action::Action,
}

#[cfg(feature = "unity_engine-application")]
#[::unity2::methods]
impl Application {
    #[method(name = "get_isPlaying", args = 0)]
    pub fn get_is_playing() -> bool;

    #[method(name = "get_dataPath", args = 0)]
    pub fn get_data_path() -> ::unity2::Il2CppString;

    #[method(name = "get_streamingAssetsPath", args = 0)]
    pub fn get_streaming_assets_path() -> ::unity2::Il2CppString;

    #[method(name = "get_persistentDataPath", args = 0)]
    pub fn get_persistent_data_path() -> ::unity2::Il2CppString;

    #[method(name = "get_temporaryCachePath", args = 0)]
    pub fn get_temporary_cache_path() -> ::unity2::Il2CppString;

    #[method(name = "get_productName", args = 0)]
    pub fn get_product_name() -> ::unity2::Il2CppString;

    #[method(name = "OpenURL", args = 1)]
    pub fn open_url(url: ::unity2::Il2CppString) -> ();

    #[method(name = "set_backgroundLoadingPriority", args = 1)]
    pub fn set_background_loading_priority(
        value: crate::unity_engine::threadpriority::ThreadPriority,
    ) -> ();

    #[method(name = "get_platform", args = 0)]
    pub fn get_platform() -> crate::unity_engine::runtimeplatform::RuntimePlatform;

    #[method(name = "get_isMobilePlatform", args = 0)]
    pub fn get_is_mobile_platform() -> bool;

    #[method(name = "CallLowMemory", args = 0)]
    pub fn call_low_memory() -> ();

    #[method(name = "CallLogCallback", args = 4)]
    pub fn call_log_callback(
        log_string: ::unity2::Il2CppString,
        stack_trace: ::unity2::Il2CppString,
        r#type: crate::unity_engine::logtype::LogType,
        invoked_on_main_thread: bool,
    ) -> ();

    #[method(name = "Internal_ApplicationWantsToQuit", args = 0)]
    pub fn internal_application_wants_to_quit() -> bool;

    #[method(name = "Internal_ApplicationQuit", args = 0)]
    pub fn internal_application_quit() -> ();

    #[method(name = "Internal_ApplicationUnload", args = 0)]
    pub fn internal_application_unload() -> ();

    #[method(name = "InvokeOnBeforeRender", args = 0)]
    pub fn invoke_on_before_render() -> ();

    #[method(name = "InvokeFocusChanged", args = 1)]
    pub fn invoke_focus_changed(focus: bool) -> ();

    #[method(name = "InvokeDeepLinkActivated", args = 1)]
    pub fn invoke_deep_link_activated(url: ::unity2::Il2CppString) -> ();

    #[method(name = "get_isEditor", args = 0)]
    pub fn get_is_editor() -> bool;
}
