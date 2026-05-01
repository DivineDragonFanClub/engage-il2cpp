
use crate::app::filecommon::FileCommon;
use crate::app::filecommon::IFileCommon;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/filehandle_1/FileHandle_1.md")))]
#[::unity2::class(namespace = "App", name = "FileHandle`1")]
pub struct FileHandle_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::filedata::FileData,
}

#[cfg(feature = "app-filehandle_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> FileHandle_1<T0> {
    #[method(name = "Load", args = 1)]
    pub fn load(self, path: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, path: ::unity2::Il2CppString) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "GetSize", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "GetPath", args = 0)]
    pub fn get_path(self) -> ::unity2::Il2CppString;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();
}

#[cfg(feature = "app-filehandle_1")]
impl<T0: ::unity2::ClassIdentity> FileHandle_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FileHandle_1),
                ::core::stringify!(new),
            )
        });
        <Self as IFileHandle_1Methods<T0>>::ctor(this);
        this
    }
}
