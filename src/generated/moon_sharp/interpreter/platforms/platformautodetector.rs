
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/platforms/platformautodetector/PlatformAutoDetector.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Platforms",
    name = "PlatformAutoDetector"
)]
#[parent(crate::system::object::Object)]
pub struct PlatformAutoDetector {
    #[static_field]
    #[rename(name = "m_AutoDetectionsDone")]
    pub m_auto_detections_done: bool,
}

#[cfg(feature = "moon_sharp-interpreter-platforms-platformautodetector")]
#[::unity2::methods]
impl PlatformAutoDetector {
    #[method(name = "get_IsRunningOnMono", args = 0)]
    pub fn get_is_running_on_mono() -> bool;

    #[method(name = "set_IsRunningOnMono", args = 1)]
    pub fn set_is_running_on_mono(value: bool) -> ();

    #[method(name = "get_IsRunningOnClr4", args = 0)]
    pub fn get_is_running_on_clr4() -> bool;

    #[method(name = "set_IsRunningOnClr4", args = 1)]
    pub fn set_is_running_on_clr4(value: bool) -> ();

    #[method(name = "get_IsRunningOnUnity", args = 0)]
    pub fn get_is_running_on_unity() -> bool;

    #[method(name = "set_IsRunningOnUnity", args = 1)]
    pub fn set_is_running_on_unity(value: bool) -> ();

    #[method(name = "get_IsPortableFramework", args = 0)]
    pub fn get_is_portable_framework() -> bool;

    #[method(name = "set_IsPortableFramework", args = 1)]
    pub fn set_is_portable_framework(value: bool) -> ();

    #[method(name = "get_IsUnityNative", args = 0)]
    pub fn get_is_unity_native() -> bool;

    #[method(name = "set_IsUnityNative", args = 1)]
    pub fn set_is_unity_native(value: bool) -> ();

    #[method(name = "get_IsUnityIL2CPP", args = 0)]
    pub fn get_is_unity_il2cpp() -> bool;

    #[method(name = "set_IsUnityIL2CPP", args = 1)]
    pub fn set_is_unity_il2cpp(value: bool) -> ();

    #[method(name = "get_IsRunningOnAOT", args = 0)]
    pub fn get_is_running_on_aot() -> bool;

    #[method(name = "AutoDetectPlatformFlags", args = 0)]
    pub fn auto_detect_platform_flags() -> ();

    #[method(name = "GetDefaultPlatform", args = 0)]
    pub fn get_default_platform(
    ) -> crate::moon_sharp::interpreter::platforms::iplatformaccessor::IPlatformAccessor;

    #[method(name = "GetDefaultScriptLoader", args = 0)]
    pub fn get_default_script_loader(
    ) -> crate::moon_sharp::interpreter::loaders::iscriptloader::IScriptLoader;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
