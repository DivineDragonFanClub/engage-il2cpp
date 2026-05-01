
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditdata/MapEditData.md")))]
#[::unity2::class(namespace = "App", name = "MapEditData")]
#[parent(crate::system::object::Object)]
pub struct MapEditData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "MaxStream")]
    pub max_stream: i32,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_SortieUnitInfoList")]
    pub m_sortie_unit_info_list: crate::system::collections::generic::list_1::List_1<
        crate::app::mapeditdata::MapEditData_UnitInfo,
    >,
    #[rename(name = "m_SortieGidList")]
    pub m_sortie_gid_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_SortieLinkGodDict")]
    pub m_sortie_link_god_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
}

#[cfg(feature = "app-mapeditdata")]
#[::unity2::methods]
impl MapEditData {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

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

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ClearValue", args = 0)]
    pub fn clear_value(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::mapeditdata::MapEditData) -> ();

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;

    #[method(name = "IsSaved", args = 0)]
    pub fn is_saved(self) -> bool;

    #[method(name = "StreamPosition", args = 0)]
    pub fn stream_position(self) -> i32;

    #[method(name = "TryGetPairGodUnit", args = 1)]
    pub fn try_get_pair_god_unit(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "WriteHeader", args = 1)]
    pub fn write_header(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteMap", args = 1)]
    pub fn write_map(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteUnit", args = 1)]
    pub fn write_unit(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteBonds", args = 1)]
    pub fn write_bonds(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "ReadHeader", args = 1)]
    pub fn read_header(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "ReadMap", args = 3)]
    pub fn read_map(
        self,
        stream: crate::app::stream_2::Stream_2,
        is_opponent: bool,
        is_append: bool,
    ) -> ();

    #[method(name = "ReadUnit", args = 1)]
    pub fn read_unit(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "ReadUnitForEnemy", args = 1)]
    pub fn read_unit_for_enemy(self, stream: crate::app::stream_2::Stream_2) -> ();

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

    #[method(name = "ReadBondsForEnemy", args = 1)]
    pub fn read_bonds_for_enemy(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> ();

    #[method(name = "LoadForEnemy", args = 0)]
    pub fn load_for_enemy(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeForMapHistoryReplay", args = 1)]
    pub fn serialize_for_map_history_replay(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeForMapHistoryReplay", args = 1)]
    pub fn deserialize_for_map_history_replay(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_SortieUnitInfoList", args = 0)]
    pub fn get_sortie_unit_info_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::mapeditdata::MapEditData_UnitInfo,
    >;

    #[method(name = "get_SortieGidList", args = 0)]
    pub fn get_sortie_gid_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "TryGetSortieUnit", args = 1)]
    pub fn try_get_sortie_unit(
        self,
        pid: ::unity2::Il2CppString,
    ) -> crate::app::mapeditdata::MapEditData_UnitInfo;

    #[method(name = "TryGetSortieGod", args = 1)]
    pub fn try_get_sortie_god(self, pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapeditdata")]
impl MapEditData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditdata/MapEditData_UnitInfo.md")))]
#[::unity2::class(namespace = "App", name = "MapEditData.UnitInfo")]
#[parent(crate::system::object::Object)]
pub struct MapEditData_UnitInfo {
    #[rename(name = "pid")]
    pub pid: ::unity2::Il2CppString,
    #[rename(name = "pos")]
    pub pos: crate::app::mappos::MapPos,
    #[rename(name = "aiType")]
    pub ai_type: crate::app::unitai::UnitAI_VersusTypes,
}

#[cfg(feature = "app-mapeditdata")]
#[::unity2::methods]
impl MapEditData_UnitInfo {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        x: i32,
        z: i32,
        ai_type: crate::app::unitai::UnitAI_VersusTypes,
    ) -> ();
}

#[cfg(feature = "app-mapeditdata")]
impl MapEditData_UnitInfo {
    pub fn new(
        pid: ::unity2::Il2CppString,
        x: i32,
        z: i32,
        ai_type: crate::app::unitai::UnitAI_VersusTypes,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditData_UnitInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditData_UnitInfoMethods>::ctor(this, pid, x, z, ai_type);
        this
    }
}
