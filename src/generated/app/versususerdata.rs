
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versususerdata/VersusUserData.md")))]
#[::unity2::class(namespace = "App", name = "VersusUserData")]
#[parent(crate::system::object::Object)]
pub struct VersusUserData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Rate")]
    pub m_rate: i16,
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
}

#[cfg(feature = "app-versususerdata")]
#[::unity2::methods]
impl VersusUserData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetUploadedInfo", args = 1)]
    pub fn set_uploaded_info(self, data_id: u64) -> ();

    #[method(name = "SetRate", args = 1)]
    pub fn set_rate(self, rate: i32) -> ();

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, mess: ::unity2::Il2CppString) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> i32;
}

#[cfg(feature = "app-versususerdata")]
impl VersusUserData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusUserData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusUserDataMethods>::ctor(this);
        this
    }
}
