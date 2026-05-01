
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugfs/DebugFs.md")))]
#[::unity2::class(namespace = "App", name = "DebugFs")]
#[parent(crate::system::object::Object)]
pub struct DebugFs {}

#[cfg(feature = "app-debugfs")]
#[::unity2::methods]
impl DebugFs {
    #[method(name = "IsExists", args = 1)]
    pub fn is_exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsFileExists", args = 1)]
    pub fn is_file_exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsDirectoryExists", args = 1)]
    pub fn is_directory_exists(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetFsEntries", args = 3)]
    pub fn get_fs_entries(
        path: ::unity2::Il2CppString,
        pattern: ::unity2::Il2CppString,
        is_recursive: bool,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetFiles", args = 3)]
    pub fn get_files(
        path: ::unity2::Il2CppString,
        pattern: ::unity2::Il2CppString,
        is_recursive: bool,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetDirectories", args = 3)]
    pub fn get_directories(
        path: ::unity2::Il2CppString,
        pattern: ::unity2::Il2CppString,
        is_recursive: bool,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "ReadFile", args = 1)]
    pub fn read_file(path: ::unity2::Il2CppString) -> ::unity2::Array<u8>;

    #[method(name = "WriteFile", args = 3)]
    pub fn write_file(path: ::unity2::Il2CppString, bin: ::unity2::Array<u8>, size: i32) -> bool;

    #[method(name = "DeleteFile", args = 1)]
    pub fn delete_file(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "CreateDirectory", args = 1)]
    pub fn create_directory(path: ::unity2::Il2CppString) -> bool;

    #[method(name = "DeleteDirectory", args = 1)]
    pub fn delete_directory(path: ::unity2::Il2CppString) -> bool;
}
