
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versususerglobaldata/VersusUserGlobalData.md")))]
#[::unity2::class(namespace = "App", name = "VersusUserGlobalData")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: versususerglobaldata :: VersusUserGlobalData >)]
pub struct VersusUserGlobalData {
    #[static_field]
    #[rename(name = "ReportedListNum")]
    pub reported_list_num: i32,
    #[rename(name = "m_ReportedList")]
    pub m_reported_list: crate::system::collections::generic::list_1::List_1<u64>,
    #[rename(name = "m_ReservedId")]
    pub m_reserved_id: u64,
}

#[cfg(feature = "app-versususerglobaldata")]
#[::unity2::methods]
impl VersusUserGlobalData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "AddReportedIDReserve", args = 1)]
    pub fn add_reported_id_reserve(self, id: u64) -> ();

    #[method(name = "ContainsReportedList", args = 1)]
    pub fn contains_reported_list(self, id: u64) -> bool;

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "AddReportedID", args = 1)]
    pub fn add_reported_id(self, id: u64) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Log", args = 1)]
    pub fn log(self, mess: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-versususerglobaldata")]
impl VersusUserGlobalData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusUserGlobalData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusUserGlobalDataMethods>::ctor(this);
        this
    }
}
