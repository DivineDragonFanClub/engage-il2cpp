
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/uploadhandler/UploadHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.Networking", name = "UploadHandler")]
#[parent(crate::system::object::Object)]
pub struct UploadHandler {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-networking-uploadhandler")]
#[::unity2::methods]
impl UploadHandler {
    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "set_contentType", args = 1)]
    pub fn set_content_type(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "InternalSetContentType", args = 1)]
    pub fn internal_set_content_type(self, new_content_type: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-networking-uploadhandler")]
impl UploadHandler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UploadHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUploadHandlerMethods>::ctor(this);
        this
    }
}
