
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusserverreportmetadata/VersusServerReportMetaData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerReportMetaData")]
#[parent(crate::system::object::Object)]
pub struct VersusServerReportMetaData {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[static_field]
    #[rename(name = "DataIdListSize")]
    pub data_id_list_size: i32,
    #[rename(name = "m_DataIdList")]
    pub m_data_id_list: crate::system::collections::generic::list_1::List_1<u64>,
}

#[cfg(feature = "app-versusserverreportmetadata")]
#[::unity2::methods]
impl VersusServerReportMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary(self, bin: ::unity2::Array<u8>) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary_2(self, bin: crate::system::collections::generic::list_1::List_1<u8>) -> ();

    #[method(name = "GetBinary", args = 0)]
    pub fn get_binary(self) -> crate::system::collections::generic::list_1::List_1<u8>;

    #[method(name = "AddDataId", args = 1)]
    pub fn add_data_id(self, data_id: u64) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, data_id: u64) -> bool;

    #[method(name = "get_Buffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "ToStringIDs", args = 0)]
    pub fn to_string_i_ds(self) -> ::unity2::Il2CppString;

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> ();
}

#[cfg(feature = "app-versusserverreportmetadata")]
impl VersusServerReportMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerReportMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerReportMetaDataMethods>::ctor(this);
        this
    }
}
