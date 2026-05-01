
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/certificatehandler/CertificateHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.Networking", name = "CertificateHandler")]
#[parent(crate::system::object::Object)]
pub struct CertificateHandler {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-networking-certificatehandler")]
#[::unity2::methods]
impl CertificateHandler {
    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "ValidateCertificate", args = 1)]
    pub fn validate_certificate(self, certificate_data: ::unity2::Array<u8>) -> bool;

    #[method(name = "ValidateCertificateNative", args = 1)]
    pub fn validate_certificate_native(self, certificate_data: ::unity2::Array<u8>) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}
