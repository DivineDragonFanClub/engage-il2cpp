
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayleavingunitdata/RelayLeavingUnitData.md")))]
#[::unity2::class(namespace = "App", name = "RelayLeavingUnitData")]
#[parent(crate::system::object::Object)]
pub struct RelayLeavingUnitData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: u16,
    #[rename(name = "m_Edit")]
    pub m_edit: crate::app::unitedit::UnitEdit,
    #[rename(name = "m_Person")]
    pub m_person: crate::app::persondata::PersonData,
    #[rename(name = "m_Job")]
    pub m_job: crate::app::jobdata::JobData,
    #[rename(name = "m_God")]
    pub m_god: crate::app::goddata::GodData,
    #[rename(name = "m_Record")]
    pub m_record: crate::app::unitrecord::UnitRecord,
    #[rename(name = "m_RelayPlayerIndex")]
    pub m_relay_player_index: u8,
    #[rename(name = "m_IsMorph")]
    pub m_is_morph: bool,
}

#[cfg(feature = "app-relayleavingunitdata")]
#[::unity2::methods]
impl RelayLeavingUnitData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Register", args = 1)]
    pub fn register(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsFemale", args = 0)]
    pub fn is_female(self) -> bool;

    #[method(name = "GetGodName", args = 0)]
    pub fn get_god_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Edit", args = 0)]
    pub fn get_edit(self) -> crate::app::unitedit::UnitEdit;

    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "get_God", args = 0)]
    pub fn get_god(self) -> crate::app::goddata::GodData;

    #[method(name = "get_Record", args = 0)]
    pub fn get_record(self) -> crate::app::unitrecord::UnitRecord;

    #[method(name = "get_RelayPlayerIndex", args = 0)]
    pub fn get_relay_player_index(self) -> u8;

    #[method(name = "get_IsMorph", args = 0)]
    pub fn get_is_morph(self) -> bool;
}

#[cfg(feature = "app-relayleavingunitdata")]
impl RelayLeavingUnitData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayLeavingUnitData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayLeavingUnitDataMethods>::ctor(this);
        this
    }
}
