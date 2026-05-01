
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/playreportsendbase/PlayReportSendBase.md")))]
#[::unity2::class(namespace = "App", name = "PlayReportSendBase")]
#[parent(crate::system::object::Object)]
pub struct PlayReportSendBase {}

#[cfg(feature = "app-playreportsendbase")]
#[::unity2::methods]
impl PlayReportSendBase {
    #[method(name = "GetBufferSize", args = 0)]
    pub fn get_buffer_size(self) -> i64;

    #[method(name = "GetBufferSizeImpl", args = 0)]
    pub fn get_buffer_size_impl(self) -> i64;

    #[method(name = "GetUndefined", args = 0)]
    pub fn get_undefined(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-playreportsendbase")]
impl PlayReportSendBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayReportSendBase),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayReportSendBaseMethods>::ctor(this);
        this
    }
}
