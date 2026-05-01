
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusservermetadata/VersusServerMetaData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerMetaData")]
#[parent(crate::system::object::Object)]
pub struct VersusServerMetaData {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_DataCode")]
    pub m_data_code: ::unity2::Il2CppString,
}

#[cfg(feature = "app-versusservermetadata")]
#[::unity2::methods]
impl VersusServerMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary(self, bin: ::unity2::Array<u8>) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary_2(self, bin: crate::system::collections::generic::list_1::List_1<u8>) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, meta_data: crate::app::versusservermetadata::VersusServerMetaData)
        -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "set_DataId", args = 1)]
    pub fn set_data_id(self, value: u64) -> ();

    #[method(name = "get_DataCode", args = 0)]
    pub fn get_data_code(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DataCode", args = 1)]
    pub fn set_data_code(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Buffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;
}

#[cfg(feature = "app-versusservermetadata")]
impl VersusServerMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerMetaDataMethods>::ctor(this);
        this
    }
}
