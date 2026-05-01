
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitreliance/UnitReliance.md")))]
#[::unity2::class(namespace = "App", name = "UnitReliance")]
#[parent(crate::system::object::Object)]
pub struct UnitReliance {
    #[static_field]
    #[rename(name = "MaxScore")]
    pub max_score: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "s_DataDict")]
    pub s_data_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::unitreliancedata::UnitRelianceData,
    >,
    #[static_field]
    #[rename(name = "s_EachDataDict")]
    pub s_each_data_dict: crate::app::eachdictionary_2::EachDictionary_2<
        i32,
        crate::app::unitreliancedata::UnitRelianceData,
    >,
}

#[cfg(feature = "app-unitreliance")]
#[::unity2::methods]
impl UnitReliance {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "ResetMapBegin", args = 0)]
    pub fn reset_map_begin() -> ();

    #[method(name = "ReflectScore", args = 0)]
    pub fn reflect_score() -> ();

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
    ) -> crate::app::unitreliancedata::UnitRelianceData;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> crate::app::unitreliancedata::UnitRelianceData;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_3(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> crate::app::unitreliancedata::UnitRelianceData;

    #[method(name = "CanAddScore", args = 2)]
    pub fn can_add_score(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAddScore", args = 3)]
    pub fn can_add_score_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        data: crate::app::unitreliancedata::UnitRelianceData,
    ) -> bool;

    #[method(name = "CanAddExpHub", args = 3)]
    pub fn can_add_exp_hub(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        data: crate::app::unitreliancedata::UnitRelianceData,
    ) -> bool;

    #[method(name = "CanAddExpHub", args = 2)]
    pub fn can_add_exp_hub_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TryAddScore", args = 3)]
    pub fn try_add_score(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        value: i32,
    ) -> bool;

    #[method(name = "TryAddExpByHub", args = 3)]
    pub fn try_add_exp_by_hub(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        value: i32,
    ) -> bool;

    #[method(name = "CanLevelUp", args = 2)]
    pub fn can_level_up(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> bool;

    #[method(name = "CanLevelUp", args = 2)]
    pub fn can_level_up_2(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> bool;

    #[method(name = "LevelUp", args = 2)]
    pub fn level_up(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "LevelUp", args = 2)]
    pub fn level_up_2(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> ();

    #[method(name = "CanBeLevelAPlus", args = 2)]
    pub fn can_be_level_a_plus(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> bool;

    #[method(name = "CanBeLevelAPlus", args = 2)]
    pub fn can_be_level_a_plus_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "SetLevelAPlus", args = 2)]
    pub fn set_level_a_plus(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "SetLevelAPlus", args = 2)]
    pub fn set_level_a_plus_2(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit)
        -> ();

    #[method(name = "TryGetLevel", args = 3)]
    pub fn try_get_level(
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> bool;

    #[method(name = "TryGetLevel", args = 3)]
    pub fn try_get_level_2(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> bool;

    #[method(name = "TryGetLevel", args = 3)]
    pub fn try_get_level_3(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> bool;

    #[method(name = "TryGetSupportData", args = 3)]
    pub fn try_get_support_data(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> crate::app::supportdata::SupportData;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "GetValidRelianceExpData", args = 2)]
    pub fn get_valid_reliance_exp_data(
        index_a: i32,
        index_b: i32,
    ) -> crate::app::relianceexpdata::RelianceExpData;

    #[method(name = "MakeDataDictKey", args = 2)]
    pub fn make_data_dict_key(
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPidFromDictKey", args = 3)]
    pub fn get_pid_from_dict_key(
        key: ::unity2::Il2CppString,
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetRelianceExpDataFromDictKey", args = 1)]
    pub fn get_reliance_exp_data_from_dict_key(
        key: ::unity2::Il2CppString,
    ) -> crate::app::relianceexpdata::RelianceExpData;

    #[method(name = "ShuffleSameScore", args = 1)]
    pub fn shuffle_same_score(
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::unitreliancemapresult::UnitRelianceMapResult,
        >,
    ) -> ();

    #[method(name = "ShuffleSameScore", args = 3)]
    pub fn shuffle_same_score_2(
        results: crate::system::collections::generic::list_1::List_1<
            crate::app::unitreliancemapresult::UnitRelianceMapResult,
        >,
        start_index: i32,
        end_index: i32,
    ) -> ();

    #[method(name = "LinkGodUnitGet", args = 1)]
    pub fn link_god_unit_get(
        person: crate::app::persondata::PersonData,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "LinkGodUnitIsExists", args = 2)]
    pub fn link_god_unit_is_exists(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> bool;

    #[method(name = "LinkGodUnitIsExists", args = 2)]
    pub fn link_god_unit_is_exists_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "LinkGodUnitLevelUp", args = 3)]
    pub fn link_god_unit_level_up(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "LinkGodUnitCanBeRelianceLevelS", args = 2)]
    pub fn link_god_unit_can_be_reliance_level_s(
        god_unit_a: crate::app::godunit::GodUnit,
        person_b: crate::app::persondata::PersonData,
    ) -> bool;

    #[method(name = "LinkGodUnitSetRelianceLevelS", args = 2)]
    pub fn link_god_unit_set_reliance_level_s(
        god_unit_a: crate::app::godunit::GodUnit,
        person_b: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitreliance")]
impl UnitReliance {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitReliance),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitRelianceMethods>::ctor(this);
        this
    }
}
