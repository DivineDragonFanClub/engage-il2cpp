
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandlinerom/DebugCommandlineRom_Options.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandlineRom.Options")]
#[parent(crate::system::object::Object)]
pub struct DebugCommandlineRom_Options {}

#[cfg(feature = "app-debugcommandlinerom")]
#[::unity2::methods]
impl DebugCommandlineRom_Options {
    #[method(name = "get_SaveDataRootPath", args = 0)]
    pub fn get_save_data_root_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SaveDataRootPath", args = 1)]
    pub fn set_save_data_root_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RelayDataRootPath", args = 0)]
    pub fn get_relay_data_root_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RelayDataRootPath", args = 1)]
    pub fn set_relay_data_root_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_VersusDataRootPath", args = 0)]
    pub fn get_versus_data_root_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_VersusDataRootPath", args = 1)]
    pub fn set_versus_data_root_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsAutoPlay", args = 0)]
    pub fn get_is_auto_play(self) -> bool;

    #[method(name = "set_IsAutoPlay", args = 1)]
    pub fn set_is_auto_play(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandlinerom")]
impl DebugCommandlineRom_Options {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandlineRom_Options),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandlineRom_OptionsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugcommandlinerom/DebugCommandlineRom.md")))]
#[::unity2::class(namespace = "App", name = "DebugCommandlineRom")]
#[parent(crate::system::object::Object)]
pub struct DebugCommandlineRom {
    #[static_field]
    #[rename(name = "s_Options")]
    pub s_options: crate::app::debugcommandlinerom::DebugCommandlineRom_Options,
}

#[cfg(feature = "app-debugcommandlinerom")]
#[::unity2::methods]
impl DebugCommandlineRom {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "get_SaveDataRootPath", args = 0)]
    pub fn get_save_data_root_path() -> ::unity2::Il2CppString;

    #[method(name = "get_RelayDataRootPath", args = 0)]
    pub fn get_relay_data_root_path() -> ::unity2::Il2CppString;

    #[method(name = "get_VersusDataRootPath", args = 0)]
    pub fn get_versus_data_root_path() -> ::unity2::Il2CppString;

    #[method(name = "get_IsAutoPlay", args = 0)]
    pub fn get_is_auto_play() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugcommandlinerom")]
impl DebugCommandlineRom {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugCommandlineRom),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugCommandlineRomMethods>::ctor(this);
        this
    }
}
