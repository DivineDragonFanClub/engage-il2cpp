
use crate::app::versusserverdata::IVersusServerData;
use crate::app::versusserverdata::VersusServerData;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusservercasualdata/VersusServerCasualData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerCasualData")]
#[parent(crate::app::versusserverdata::VersusServerData)]
pub struct VersusServerCasualData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
}

#[cfg(feature = "app-versusservercasualdata")]
#[::unity2::methods]
impl VersusServerCasualData {
    #[method(name = "get_UnitList", args = 0)]
    pub fn get_unit_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>;

    #[method(name = "get_m_GodDict", args = 0)]
    pub fn get_m_god_dict(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::godunit::GodUnit,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "TryGetPairGodUnit", args = 1)]
    pub fn try_get_pair_god_unit(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "SerializeInit", args = 0)]
    pub fn serialize_init(self) -> ();

    #[method(name = "WriteHeader", args = 1)]
    pub fn write_header(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteUnit", args = 1)]
    pub fn write_unit(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteUnitInit", args = 1)]
    pub fn write_unit_init(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteUnit", args = 2)]
    pub fn write_unit_2(
        self,
        stream: crate::app::stream_2::Stream_2,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "WriteBonds", args = 1)]
    pub fn write_bonds(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "ReadHeader", args = 1)]
    pub fn read_header(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "ReadUnit", args = 1)]
    pub fn read_unit(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SaveGodBond", args = 0)]
    pub fn save_god_bond(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >;

    #[method(name = "LoadGodBond", args = 1)]
    pub fn load_god_bond(
        self,
        bond_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    ) -> ();

    #[method(name = "ReadBonds", args = 1)]
    pub fn read_bonds(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-versusservercasualdata")]
impl VersusServerCasualData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerCasualData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerCasualDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusservercasualdata/VersusServerCasualData_GodBondBackup.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerCasualData.GodBondBackup")]
#[parent(crate::system::object::Object)]
pub struct VersusServerCasualData_GodBondBackup {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_BondsDict")]
    pub m_bonds_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
}

#[cfg(feature = "app-versusservercasualdata")]
#[::unity2::methods]
impl VersusServerCasualData_GodBondBackup {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "Restore", args = 0)]
    pub fn restore(self) -> ();
}

#[cfg(feature = "app-versusservercasualdata")]
impl VersusServerCasualData_GodBondBackup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerCasualData_GodBondBackup),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerCasualData_GodBondBackupMethods>::ctor(this);
        this
    }
}
