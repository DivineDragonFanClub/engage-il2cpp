
use crate::app::linknode_1::ILinkNode_1;
use crate::app::linknode_1::LinkNode_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godbondholder/GodBondHolder.md")))]
#[::unity2::class(namespace = "App", name = "GodBondHolder")]
# [parent (crate :: app :: linknode_1 :: LinkNode_1 < crate :: app :: godbondholder :: GodBondHolder >)]
pub struct GodBondHolder {
    #[static_field]
    #[rename(name = "MaxUnit")]
    pub max_unit: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::goddata::GodData,
    #[rename(name = "m_RelianceS")]
    pub m_reliance_s: crate::app::godreliances::GodRelianceS,
    #[rename(name = "m_Bonds")]
    pub m_bonds: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::godbond::GodBond,
    >,
    #[rename(name = "m_Pool")]
    pub m_pool: crate::app::pool::Pool_List_1<crate::app::godbond::GodBond>,
}

#[cfg(feature = "app-godbondholder")]
#[::unity2::methods]
impl GodBondHolder {
    #[method(name = "Build", args = 1)]
    pub fn build(
        self,
        data: crate::app::goddata::GodData,
    ) -> crate::app::godbondholder::GodBondHolder;

    #[method(name = "Get", args = 1)]
    pub fn get(self, unit: crate::app::unit::Unit) -> crate::app::godbond::GodBond;

    #[method(name = "Create", args = 1)]
    pub fn create(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateAllBondsForGodState", args = 1)]
    pub fn update_all_bonds_for_god_state(self, god_state: crate::app::godstate::GodState) -> ();

    #[method(name = "DeleteExluding", args = 1)]
    pub fn delete_exluding(
        self,
        pids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "GetMaxLevel", args = 0)]
    pub fn get_max_level(self) -> i32;

    #[method(name = "GetCountOfRelianceLevelA", args = 0)]
    pub fn get_count_of_reliance_level_a(self) -> i32;

    #[method(name = "NewGodBond", args = 1)]
    pub fn new_god_bond(self, pid: ::unity2::Il2CppString) -> crate::app::godbond::GodBond;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetLevelFromUnitRelianceIfPossible", args = 1)]
    pub fn set_level_from_unit_reliance_if_possible(self, data: crate::app::goddata::GodData)
        -> ();

    #[method(name = "SetLevelFromUnitReliance", args = 2)]
    pub fn set_level_from_unit_reliance(
        self,
        link_unit: crate::app::unit::Unit,
        force_type: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "ChangeOpponent", args = 1)]
    pub fn change_opponent(self, data: crate::app::goddata::GodData) -> ();

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "SerializeForRewindLatest", args = 2)]
    pub fn serialize_for_rewind_latest(
        self,
        stream: crate::app::stream_2::Stream_2,
        exclude_pids: crate::system::collections::generic::hashset_1::HashSet_1<
            ::unity2::Il2CppString,
        >,
    ) -> ();

    #[method(name = "DeserializeForRewindLatest", args = 1)]
    pub fn deserialize_for_rewind_latest(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::goddata::GodData;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godbondholder")]
impl GodBondHolder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodBondHolder),
                ::core::stringify!(new),
            )
        });
        <Self as IGodBondHolderMethods>::ctor(this);
        this
    }
}
