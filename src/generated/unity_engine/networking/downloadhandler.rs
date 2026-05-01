
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/downloadhandler/DownloadHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.Networking", name = "DownloadHandler")]
#[parent(crate::system::object::Object)]
pub struct DownloadHandler {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-networking-downloadhandler")]
#[::unity2::methods]
impl DownloadHandler {
    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_error", args = 0)]
    pub fn get_error(self) -> ::unity2::Il2CppString;

    #[method(name = "GetErrorMsg", args = 0)]
    pub fn get_error_msg(self) -> ::unity2::Il2CppString;

    #[method(name = "get_data", args = 0)]
    pub fn get_data(self) -> ::unity2::Array<u8>;

    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetContentType", args = 0)]
    pub fn get_content_type(self) -> ::unity2::Il2CppString;

    #[method(name = "ReceiveContentLengthHeader", args = 1)]
    pub fn receive_content_length_header(self, content_length: u64) -> ();

    #[method(name = "ReceiveContentLength", args = 1)]
    pub fn receive_content_length(self, content_length: i32) -> ();

    #[method(name = "InternalGetByteArray", args = 1)]
    pub fn internal_get_byte_array(
        dh: crate::unity_engine::networking::downloadhandler::DownloadHandler,
    ) -> ::unity2::Array<u8>;
}

#[cfg(feature = "unity_engine-networking-downloadhandler")]
impl DownloadHandler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DownloadHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IDownloadHandlerMethods>::ctor(this);
        this
    }
}
