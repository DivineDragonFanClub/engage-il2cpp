
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debughost/DebugHost.md")))]
#[::unity2::class(namespace = "App", name = "DebugHost")]
#[parent(crate::system::object::Object)]
pub struct DebugHost {}

#[cfg(feature = "app-debughost")]
#[::unity2::methods]
impl DebugHost {
    #[method(name = "Setup", args = 1)]
    pub fn setup(root: ::unity2::Il2CppString) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup() -> ();

    #[method(name = "HasEnv", args = 1)]
    pub fn has_env(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetEnv", args = 1)]
    pub fn get_env(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetUserName", args = 0)]
    pub fn get_user_name() -> ::unity2::Il2CppString;

    #[method(name = "get_IsConnected", args = 0)]
    pub fn get_is_connected() -> bool;

    #[method(name = "get_Root", args = 0)]
    pub fn get_root() -> ::unity2::Il2CppString;

    #[method(name = "WriteToHost", args = 2)]
    pub fn write_to_host(path: ::unity2::Il2CppString, text: ::unity2::Il2CppString) -> bool;

    #[method(name = "WriteToHost", args = 2)]
    pub fn write_to_host_2(path: ::unity2::Il2CppString, data: ::unity2::Array<u8>) -> bool;

    #[method(name = "WriteToSdCard", args = 2)]
    pub fn write_to_sd_card(path: ::unity2::Il2CppString, text: ::unity2::Il2CppString) -> bool;

    #[method(name = "WriteToSdCard", args = 2)]
    pub fn write_to_sd_card_2(path: ::unity2::Il2CppString, data: ::unity2::Array<u8>) -> bool;

    #[method(name = "Exists", args = 1)]
    pub fn exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "Load", args = 1)]
    pub fn load(path: ::unity2::Il2CppString) -> ::unity2::Array<u8>;
}
