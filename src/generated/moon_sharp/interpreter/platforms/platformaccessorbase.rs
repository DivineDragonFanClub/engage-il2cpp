
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/platforms/platformaccessorbase/PlatformAccessorBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Platforms",
    name = "PlatformAccessorBase"
)]
#[parent(crate::system::object::Object)]
pub struct PlatformAccessorBase {}

#[cfg(feature = "moon_sharp-interpreter-platforms-platformaccessorbase")]
#[::unity2::methods]
impl PlatformAccessorBase {
    #[method(name = "GetPlatformNamePrefix", args = 0)]
    pub fn get_platform_name_prefix(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPlatformName", args = 0)]
    pub fn get_platform_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetUnityRuntimeName", args = 0)]
    pub fn get_unity_runtime_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetUnityPlatformName", args = 0)]
    pub fn get_unity_platform_name(self) -> ::unity2::Il2CppString;

    #[method(name = "DefaultPrint", args = 1)]
    pub fn default_print(self, content: ::unity2::Il2CppString) -> ();

    #[method(name = "DefaultInput", args = 0)]
    pub fn default_input(self) -> ::unity2::Il2CppString;

    #[method(name = "DefaultInput", args = 1)]
    pub fn default_input_2(self, prompt: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "IO_GetStandardStream", args = 1)]
    pub fn io_get_standard_stream(
        self,
        r#type: crate::moon_sharp::interpreter::platforms::standardfiletype::StandardFileType,
    ) -> crate::system::io::stream::Stream;

    #[method(name = "IO_OS_GetTempFilename", args = 0)]
    pub fn io_os_get_temp_filename(self) -> ::unity2::Il2CppString;

    #[method(name = "OS_ExitFast", args = 1)]
    pub fn os_exit_fast(self, exit_code: i32) -> ();

    #[method(name = "OS_FileExists", args = 1)]
    pub fn os_file_exists(self, file: ::unity2::Il2CppString) -> bool;

    #[method(name = "OS_FileDelete", args = 1)]
    pub fn os_file_delete(self, file: ::unity2::Il2CppString) -> ();

    #[method(name = "OS_FileMove", args = 2)]
    pub fn os_file_move(self, src: ::unity2::Il2CppString, dst: ::unity2::Il2CppString) -> ();

    #[method(name = "OS_Execute", args = 1)]
    pub fn os_execute(self, cmdline: ::unity2::Il2CppString) -> i32;

    #[method(name = "FilterSupportedCoreModules", args = 1)]
    pub fn filter_supported_core_modules(
        self,
        module: crate::moon_sharp::interpreter::coremodules::CoreModules,
    ) -> crate::moon_sharp::interpreter::coremodules::CoreModules;

    #[method(name = "GetEnvironmentVariable", args = 1)]
    pub fn get_environment_variable(
        self,
        envvarname: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsRunningOnAOT", args = 0)]
    pub fn is_running_on_aot(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-platforms-platformaccessorbase")]
impl PlatformAccessorBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlatformAccessorBase),
                ::core::stringify!(new),
            )
        });
        <Self as IPlatformAccessorBaseMethods>::ctor(this);
        this
    }
}
