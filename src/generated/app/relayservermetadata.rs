
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayservermetadata/RelayServerMetaData.md")))]
#[::unity2::class(namespace = "App", name = "RelayServerMetaData")]
#[parent(crate::system::object::Object)]
pub struct RelayServerMetaData {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[static_field]
    #[rename(name = "BufferOnServerSize")]
    pub buffer_on_server_size: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_BufferOnServer")]
    pub m_buffer_on_server: ::unity2::Array<u8>,
    #[rename(name = "m_SizeOnServer")]
    pub m_size_on_server: i32,
    #[rename(name = "m_DataId")]
    pub m_data_id: u64,
    #[rename(name = "m_DataCode")]
    pub m_data_code: ::unity2::Il2CppString,
    #[rename(name = "m_UpdatedUnixTimeOnServer")]
    pub m_updated_unix_time_on_server: i64,
    #[rename(name = "m_IsSecret")]
    pub m_is_secret: bool,
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
    #[rename(name = "m_BeginUnixTime")]
    pub m_begin_unix_time: i64,
    #[rename(name = "m_EndUnixTime")]
    pub m_end_unix_time: i64,
    #[rename(name = "m_Turn")]
    pub m_turn: u8,
    #[rename(name = "m_State")]
    pub m_state: crate::app::relayservermetadata::RelayServerMetaData_States,
    #[rename(name = "m_PlayerIds")]
    pub m_player_ids: crate::system::collections::generic::list_1::List_1<u64>,
    #[rename(name = "m_PlayerNames")]
    pub m_player_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_Pids")]
    pub m_pids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_PlayingCount")]
    pub m_playing_count: u32,
    #[rename(name = "m_Awardees")]
    pub m_awardees: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::relayawardeedata::RelayAwardeeData,
    >,
    #[rename(name = "m_AwardRandomSeed")]
    pub m_award_random_seed: crate::app::randomseed::RandomSeed,
}

#[cfg(feature = "app-relayservermetadata")]
#[::unity2::methods]
impl RelayServerMetaData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::app::relayservermetadata::RelayServerMetaData;

    #[method(name = "SetOwner", args = 2)]
    pub fn set_owner(self, principal_id: u64, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetCid", args = 1)]
    pub fn set_cid(self, cid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary(self, bin: ::unity2::Array<u8>) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary_2(self, bin: crate::system::collections::generic::list_1::List_1<u8>) -> ();

    #[method(name = "UpdateBufferOnServer", args = 0)]
    pub fn update_buffer_on_server(self) -> ();

    #[method(name = "Begin", args = 0)]
    pub fn begin(self) -> ();

    #[method(name = "Complete", args = 0)]
    pub fn complete(self) -> ();

    #[method(name = "GameOver", args = 0)]
    pub fn game_over(self) -> ();

    #[method(name = "ComplementTime", args = 1)]
    pub fn complement_time(self, unix_time: i64) -> ();

    #[method(name = "AddPlayer", args = 2)]
    pub fn add_player(self, principal_id: u64, name: ::unity2::Il2CppString) -> ();

    #[method(name = "AddPid", args = 1)]
    pub fn add_pid(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "GetOwnerName", args = 0)]
    pub fn get_owner_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPlayerName", args = 1)]
    pub fn get_player_name(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "CreateAwardRandom", args = 0)]
    pub fn create_award_random(self) -> crate::app::random_2::Random_2;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "get_Buffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "get_BufferOnServer", args = 0)]
    pub fn get_buffer_on_server(self) -> ::unity2::Array<u8>;

    #[method(name = "get_SizeOnServer", args = 0)]
    pub fn get_size_on_server(self) -> i32;

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "set_DataId", args = 1)]
    pub fn set_data_id(self, value: u64) -> ();

    #[method(name = "get_DataCode", args = 0)]
    pub fn get_data_code(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DataCode", args = 1)]
    pub fn set_data_code(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UpdatedUnixTimeOnServer", args = 0)]
    pub fn get_updated_unix_time_on_server(self) -> i64;

    #[method(name = "set_UpdatedUnixTimeOnServer", args = 1)]
    pub fn set_updated_unix_time_on_server(self, value: i64) -> ();

    #[method(name = "get_IsSecret", args = 0)]
    pub fn get_is_secret(self) -> bool;

    #[method(name = "set_IsSecret", args = 1)]
    pub fn set_is_secret(self, value: bool) -> ();

    #[method(name = "get_OwnerId", args = 0)]
    pub fn get_owner_id(self) -> u64;

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_BeginUnixTime", args = 0)]
    pub fn get_begin_unix_time(self) -> i64;

    #[method(name = "get_EndUnixTime", args = 0)]
    pub fn get_end_unix_time(self) -> i64;

    #[method(name = "get_Turn", args = 0)]
    pub fn get_turn(self) -> i32;

    #[method(name = "set_Turn", args = 1)]
    pub fn set_turn(self, value: i32) -> ();

    #[method(name = "get_IsEnd", args = 0)]
    pub fn get_is_end(self) -> bool;

    #[method(name = "get_IsCompleted", args = 0)]
    pub fn get_is_completed(self) -> bool;

    #[method(name = "get_PlayerIds", args = 0)]
    pub fn get_player_ids(self) -> crate::system::collections::generic::list_1::List_1<u64>;

    #[method(name = "get_PlayerNames", args = 0)]
    pub fn get_player_names(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_Pids", args = 0)]
    pub fn get_pids(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_PlayingCount", args = 0)]
    pub fn get_playing_count(self) -> u32;

    #[method(name = "set_PlayingCount", args = 1)]
    pub fn set_playing_count(self, value: u32) -> ();

    #[method(name = "get_Awardees", args = 0)]
    pub fn get_awardees(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::relayawardeedata::RelayAwardeeData,
    >;

    #[method(name = "set_Awardees", args = 1)]
    pub fn set_awardees(
        self,
        value: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::relayawardeedata::RelayAwardeeData,
        >,
    ) -> ();

    #[method(name = "TrySetBeginUnixTime", args = 1)]
    pub fn try_set_begin_unix_time(self, unix_time: i64) -> ();

    #[method(name = "TrySetEndUnixTime", args = 1)]
    pub fn try_set_end_unix_time(self, unix_time: i64) -> ();

    #[method(name = "SetAwardRandomSeed", args = 0)]
    pub fn set_award_random_seed(self) -> ();
}

#[cfg(feature = "app-relayservermetadata")]
impl RelayServerMetaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayServerMetaData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayServerMetaDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayservermetadata/RelayServerMetaData_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayServerMetaData_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayServerMetaData_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayServerMetaData.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayServerMetaData_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayServerMetaData_States {
    pub fn progressing() -> Self {
        Self { value: 0 }
    }

    pub fn completed() -> Self {
        Self { value: 1 }
    }

    pub fn game_over() -> Self {
        Self { value: 2 }
    }
}
