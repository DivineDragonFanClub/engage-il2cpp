
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusserverreplaymetadata/VersusServerReplayMetaData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerReplayMetaData")]
#[parent(crate::system::object::Object)]
pub struct VersusServerReplayMetaData {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_Version")]
    pub m_version: u16,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_OffensePrincipalId")]
    pub m_offense_principal_id: u64,
    #[rename(name = "m_OffensePlayerName")]
    pub m_offense_player_name: ::unity2::Il2CppString,
    #[rename(name = "m_MapEndUnixTime")]
    pub m_map_end_unix_time: i64,
    #[rename(name = "m_MapResult")]
    pub m_map_result: crate::app::versus::Versus_MapResult,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
    #[rename(name = "m_IsEmpty")]
    pub m_is_empty: bool,
}

#[cfg(feature = "app-versusserverreplaymetadata")]
#[::unity2::methods]
impl VersusServerReplayMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary(self, bin: ::unity2::Array<u8>) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary_2(self, bin: crate::system::collections::generic::list_1::List_1<u8>) -> ();

    #[method(name = "SetMapResult", args = 1)]
    pub fn set_map_result(self, result: crate::app::versus::Versus_MapResult) -> ();

    #[method(name = "TrySetMapEndTime", args = 1)]
    pub fn try_set_map_end_time(self, unix_time: i64) -> ();

    #[method(name = "SetOffensePlayer", args = 2)]
    pub fn set_offense_player(
        self,
        principal_id: u64,
        offense_player_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, opt: ::unity2::Il2CppString) -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "get_Buffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "set_DataId", args = 1)]
    pub fn set_data_id(self, value: u64) -> ();

    #[method(name = "get_IsEmpty", args = 0)]
    pub fn get_is_empty(self) -> bool;

    #[method(name = "get_OffensePrincipalId", args = 0)]
    pub fn get_offense_principal_id(self) -> u64;

    #[method(name = "get_OffensePlayerName", args = 0)]
    pub fn get_offense_player_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_MapEndUnixTime", args = 0)]
    pub fn get_map_end_unix_time(self) -> i64;

    #[method(name = "get_MapEndUnixTimeStr", args = 0)]
    pub fn get_map_end_unix_time_str(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Result", args = 0)]
    pub fn get_result(self) -> crate::app::versus::Versus_MapResult;

    #[method(name = "get_ResultForDefense", args = 0)]
    pub fn get_result_for_defense(self) -> crate::app::versus::Versus_MapResult;

    #[method(name = "get_IsEnable", args = 0)]
    pub fn get_is_enable(self) -> bool;

    #[method(name = "get_IsClearReliance", args = 0)]
    pub fn get_is_clear_reliance(self) -> bool;

    #[method(name = "TryGetPlayableMetaData", args = 1)]
    pub fn try_get_playable_meta_data(
        meta_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        >,
    ) -> crate::app::versusserverreplaymetadata::VersusServerReplayMetaData;

    #[method(name = "GetUploadTargetMetaData", args = 1)]
    pub fn get_upload_target_meta_data(
        meta_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        >,
    ) -> crate::app::versusserverreplaymetadata::VersusServerReplayMetaData;
}

#[cfg(feature = "app-versusserverreplaymetadata")]
impl VersusServerReplayMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerReplayMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerReplayMetaDataMethods>::ctor(this);
        this
    }
}
