
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/binlibmsfilebase/BinLibmsFileBase.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "BinLibmsFileBase")]
#[parent(crate::system::object::Object)]
pub struct BinLibmsFileBase {
    #[rename(name = "m_ResourceFilePtr")]
    pub m_resource_file_ptr: ::unity2::IntPtr,
    #[rename(name = "m_FileObjectPtr")]
    pub m_file_object_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "nintendo-message_studio-lib-binlibmsfilebase")]
#[::unity2::methods]
impl BinLibmsFileBase {
    #[method(name = "get_FileObjectPtr", args = 0)]
    pub fn get_file_object_ptr(self) -> ::unity2::IntPtr;

    #[method(name = "get_IsFileLoaded", args = 0)]
    pub fn get_is_file_loaded(self) -> bool;

    #[method(name = "Load", args = 1)]
    pub fn load(self, bytes: ::unity2::Array<u8>) -> ();

    #[method(name = "Free", args = 0)]
    pub fn free(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "InitObject", args = 1)]
    pub fn init_object(self, resource_ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "CloseObject", args = 1)]
    pub fn close_object(self, object_ptr: ::unity2::IntPtr) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-binlibmsfilebase")]
impl BinLibmsFileBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BinLibmsFileBase),
                ::core::stringify!(new),
            )
        });
        <Self as IBinLibmsFileBaseMethods>::ctor(this);
        this
    }
}
