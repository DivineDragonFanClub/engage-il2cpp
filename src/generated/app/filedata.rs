
use crate::app::filecommon::FileCommon;
use crate::app::filecommon::IFileCommon;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/filedata/FileData.md")))]
#[::unity2::class(namespace = "App", name = "FileData")]
#[parent(crate::app::filecommon::FileCommon)]
pub struct FileData {
    #[rename(name = "m_State")]
    pub m_state: crate::app::filecommon::FileCommon_State,
    #[rename(name = "m_Path")]
    pub m_path: ::unity2::Il2CppString,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<u8>,
    #[rename(name = "m_Refer")]
    pub m_refer: crate::app::bindholder::BindHolder,
}

#[cfg(feature = "app-filedata")]
#[::unity2::methods]
impl FileData {
    #[method(name = "GetPath", args = 0)]
    pub fn get_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "GetSize", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, index: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetCrc32", args = 0)]
    pub fn get_crc32(self) -> u32;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;
}

#[cfg(feature = "app-filedata")]
impl FileData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FileData),
                ::core::stringify!(new),
            )
        });
        <Self as IFileDataMethods>::ctor(this);
        this
    }
}
