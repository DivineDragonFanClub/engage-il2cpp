
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawardeedata/RelayAwardeeData.md")))]
#[::unity2::class(namespace = "App", name = "RelayAwardeeData")]
#[parent(crate::system::object::Object)]
pub struct RelayAwardeeData {
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
    #[rename(name = "m_RecordValue")]
    pub m_record_value: i32,
    #[rename(name = "m_RelayPlayerIndex")]
    pub m_relay_player_index: u8,
    #[rename(name = "m_IsMorph")]
    pub m_is_morph: bool,
    #[rename(name = "m_Item")]
    pub m_item: crate::app::itemdata::ItemData,
}

#[cfg(feature = "app-relayawardeedata")]
#[::unity2::methods]
impl RelayAwardeeData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Register", args = 3)]
    pub fn register(
        self,
        unit: crate::app::unit::Unit,
        record_value: i32,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "Register", args = 3)]
    pub fn register_2(
        self,
        leaving_unit_data: crate::app::relayleavingunitdata::RelayLeavingUnitData,
        record_value: i32,
        item: crate::app::itemdata::ItemData,
    ) -> ();

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

    #[method(name = "get_RecordValue", args = 0)]
    pub fn get_record_value(self) -> i32;

    #[method(name = "get_RelayPlayerIndex", args = 0)]
    pub fn get_relay_player_index(self) -> u8;

    #[method(name = "get_IsMorth", args = 0)]
    pub fn get_is_morth(self) -> bool;

    #[method(name = "get_Item", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;
}

#[cfg(feature = "app-relayawardeedata")]
impl RelayAwardeeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAwardeeData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayAwardeeDataMethods>::ctor(this);
        this
    }
}
