
use crate::app::filecommon::FileCommon;
use crate::app::filecommon::IFileCommon;
use crate::app::filehandle_1::FileHandle_1;
use crate::app::filehandle_1::IFileHandle_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rawfilehandle/RawFileHandle.md")))]
#[::unity2::class(namespace = "App", name = "RawFileHandle")]
# [parent (crate :: app :: filehandle_1 :: FileHandle_1 < crate :: app :: filedata :: FileData >)]
pub struct RawFileHandle {}

#[cfg(feature = "app-rawfilehandle")]
#[::unity2::methods]
impl RawFileHandle {
    #[method(name = "GetTextUTF8", args = 0)]
    pub fn get_text_utf8(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rawfilehandle")]
impl RawFileHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RawFileHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IRawFileHandleMethods>::ctor(this);
        this
    }
}
