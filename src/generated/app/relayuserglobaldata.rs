
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserglobaldata/RelayUserGlobalData_Uncommitted.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RelayUserGlobalData_Uncommitted {
    pub ticket_count: i32,
    pub ticket_last_time: i64,
    pub is_valid_ticket_last_time: bool,
}

impl ::unity2::ClassIdentity for RelayUserGlobalData_Uncommitted {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayUserGlobalData.Uncommitted";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayUserGlobalData_Uncommitted {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-relayuserglobaldata")]
#[::unity2::methods(value)]
impl RelayUserGlobalData_Uncommitted {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsDirty", args = 0)]
    pub fn is_dirty(self) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserglobaldata/RelayUserGlobalData.md")))]
#[::unity2::class(namespace = "App", name = "RelayUserGlobalData")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: relayuserglobaldata :: RelayUserGlobalData >)]
pub struct RelayUserGlobalData {
    #[static_field]
    #[rename(name = "MaxTicketCount")]
    pub max_ticket_count: u32,
    #[static_field]
    #[rename(name = "InitTicketLastTime")]
    pub init_ticket_last_time: i64,
    #[static_field]
    #[rename(name = "InitDailyTicketCount")]
    pub init_daily_ticket_count: i32,
    #[rename(name = "m_DailyTicketCount")]
    pub m_daily_ticket_count: u32,
    #[rename(name = "m_TicketDict")]
    pub m_ticket_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
        u64,
        crate::app::relayuserglobaldata::RelayUserGlobalData_Ticket,
    >,
    #[rename(name = "m_Uncommitted")]
    pub m_uncommitted: crate::app::relayuserglobaldata::RelayUserGlobalData_Uncommitted,
}

#[cfg(feature = "app-relayuserglobaldata")]
#[::unity2::methods]
impl RelayUserGlobalData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ClearForGame", args = 0)]
    pub fn clear_for_game(self) -> ();

    #[method(name = "CleanupUnused", args = 1)]
    pub fn cleanup_unused(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "UpdateData", args = 0)]
    pub fn update_data(self) -> ();

    #[method(name = "DeleteData", args = 1)]
    pub fn delete_data(self, identifier: u64) -> ();

    #[method(name = "AddTicketCount", args = 1)]
    pub fn add_ticket_count(self, count: i32) -> ();

    #[method(name = "SubTicketCount", args = 1)]
    pub fn sub_ticket_count(self, count: i32) -> ();

    #[method(name = "GetTicketCount", args = 0)]
    pub fn get_ticket_count(self) -> u32;

    #[method(name = "SetTicketLastTime", args = 1)]
    pub fn set_ticket_last_time(self, unix_time: i64) -> ();

    #[method(name = "GetTicketLastTime", args = 0)]
    pub fn get_ticket_last_time(self) -> i64;

    #[method(name = "GetTicketCurrentTime", args = 0)]
    pub fn get_ticket_current_time() -> i64;

    #[method(name = "get_DailyTicketCount", args = 0)]
    pub fn get_daily_ticket_count(self) -> u32;

    #[method(name = "set_DailyTicketCount", args = 1)]
    pub fn set_daily_ticket_count(self, value: u32) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "SerializeUncommitted", args = 1)]
    pub fn serialize_uncommitted(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeUncommitted", args = 1)]
    pub fn deserialize_uncommitted(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "ClearUncommitted", args = 0)]
    pub fn clear_uncommitted(self) -> ();

    #[method(name = "DbgDump", args = 1)]
    pub fn dbg_dump(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "DbgAddSubTicketCountTest", args = 0)]
    pub fn dbg_add_sub_ticket_count_test(self) -> ();
}

#[cfg(feature = "app-relayuserglobaldata")]
impl RelayUserGlobalData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserGlobalData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUserGlobalDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserglobaldata/RelayUserGlobalData_Ticket.md")))]
#[::unity2::class(namespace = "App", name = "RelayUserGlobalData.Ticket")]
#[parent(crate::system::object::Object)]
pub struct RelayUserGlobalData_Ticket {}

#[cfg(feature = "app-relayuserglobaldata")]
#[::unity2::methods]
impl RelayUserGlobalData_Ticket {
    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> u32;

    #[method(name = "set_Count", args = 1)]
    pub fn set_count(self, value: u32) -> ();

    #[method(name = "get_LastTime", args = 0)]
    pub fn get_last_time(self) -> i64;

    #[method(name = "set_LastTime", args = 1)]
    pub fn set_last_time(self, value: i64) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 2)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayuserglobaldata")]
impl RelayUserGlobalData_Ticket {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserGlobalData_Ticket),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUserGlobalData_TicketMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserglobaldata/RelayUserGlobalData_ProcCleanupUnused.md")))]
#[::unity2::class(namespace = "App", name = "RelayUserGlobalData.ProcCleanupUnused")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RelayUserGlobalData_ProcCleanupUnused {
    #[rename(name = "m_HeaderReader")]
    pub m_header_reader: crate::app::gamesavedataheaderreader::GameSaveDataHeaderReader,
}

#[cfg(feature = "app-relayuserglobaldata")]
#[::unity2::methods]
impl RelayUserGlobalData_ProcCleanupUnused {
    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "ReadHeader", args = 0)]
    pub fn read_header(self) -> ();

    #[method(name = "IsReadingHeader", args = 0)]
    pub fn is_reading_header(self) -> bool;

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayuserglobaldata")]
impl RelayUserGlobalData_ProcCleanupUnused {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserGlobalData_ProcCleanupUnused),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUserGlobalData_ProcCleanupUnusedMethods>::ctor(this);
        this
    }
}
