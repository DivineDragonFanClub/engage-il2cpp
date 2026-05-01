
use crate::app::linknode_1::ILinkNode_1;
use crate::app::linknode_1::LinkNode_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godunit/GodUnit.md")))]
#[::unity2::class(namespace = "App", name = "GodUnit")]
# [parent (crate :: app :: linknode_1 :: LinkNode_1 < crate :: app :: godunit :: GodUnit >)]
pub struct GodUnit {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "MaxSynchroCount")]
    pub max_synchro_count: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::goddata::GodData,
    #[rename(name = "m_Parent")]
    pub m_parent: crate::app::unit::Unit,
    #[rename(name = "m_Child")]
    pub m_child: crate::app::unit::Unit,
    #[rename(name = "m_Bonds")]
    pub m_bonds: crate::app::godbondholder::GodBondHolder,
    #[rename(name = "m_SavedParent")]
    pub m_saved_parent: crate::app::unit::Unit,
    #[rename(name = "m_IsTemporaryParent")]
    pub m_is_temporary_parent: bool,
    #[rename(name = "m_IsTemporaryChanged")]
    pub m_is_temporary_changed: bool,
    #[rename(name = "m_IsReservedDeleting")]
    pub m_is_reserved_deleting: bool,
    #[rename(name = "m_IsEscaping")]
    pub m_is_escaping: bool,
    #[rename(name = "m_Dirty")]
    pub m_dirty: u8,
    #[rename(name = "m_SynchroCounts")]
    pub m_synchro_counts: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
    #[rename(name = "m_WeaponRefineLevels")]
    pub m_weapon_refine_levels: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::godweaponrefinelevels::GodWeaponRefineLevels,
    >,
    #[rename(name = "m_WeaponRefineResults")]
    pub m_weapon_refine_results: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::godweaponrefineresult::GodWeaponRefineResult,
    >,
    #[rename(name = "m_SyncroSkills")]
    pub m_syncro_skills: crate::app::skillarray::SkillArray,
    #[static_field]
    #[rename(name = "DirtyMax")]
    pub dirty_max: i32,
    #[static_field]
    #[rename(name = "s_NullItems")]
    pub s_null_items:
        crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    #[static_field]
    #[rename(name = "s_NullSkills")]
    pub s_null_skills: crate::app::skillarray::SkillArray,
}

#[cfg(feature = "app-godunit")]
#[::unity2::methods]
impl GodUnit {
    #[method(name = "Build", args = 1)]
    pub fn build(self, data: crate::app::goddata::GodData) -> crate::app::godunit::GodUnit;

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "ReserveDeleting", args = 0)]
    pub fn reserve_deleting(self) -> ();

    #[method(name = "CancelToReserveDeleting", args = 0)]
    pub fn cancel_to_reserve_deleting(self) -> ();

    #[method(name = "get_IsReservedDeleting", args = 0)]
    pub fn get_is_reserved_deleting(self) -> bool;

    #[method(name = "get_IsEscaping", args = 0)]
    pub fn get_is_escaping(self) -> bool;

    #[method(name = "set_IsEscaping", args = 1)]
    pub fn set_is_escaping(self, value: bool) -> ();

    #[method(name = "get_Dirty", args = 0)]
    pub fn get_dirty(self) -> i32;

    #[method(name = "set_Dirty", args = 1)]
    pub fn set_dirty(self, value: i32) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::goddata::GodData;

    #[method(name = "get_InfoData", args = 0)]
    pub fn get_info_data(self) -> crate::app::goddata::GodData;

    #[method(name = "get_ActualData", args = 0)]
    pub fn get_actual_data(self) -> crate::app::goddata::GodData;

    #[method(name = "get_ResultData", args = 0)]
    pub fn get_result_data(self) -> crate::app::goddata::GodData;

    #[method(name = "get_Gid", args = 0)]
    pub fn get_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActualGid", args = 0)]
    pub fn get_actual_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "IsHero", args = 0)]
    pub fn is_hero(self) -> bool;

    #[method(name = "IsFlag", args = 1)]
    pub fn is_flag(self, flag: crate::app::goddata::GodData_Flags) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetInfoName", args = 0)]
    pub fn get_info_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMainName", args = 0)]
    pub fn get_main_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 1)]
    pub fn get_name_2(data: crate::app::goddata::GodData) -> ::unity2::Il2CppString;

    #[method(name = "GetAsciiName", args = 0)]
    pub fn get_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMainAsciiName", args = 0)]
    pub fn get_main_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFaceIconName", args = 0)]
    pub fn get_face_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetInfoFaceIconName", args = 0)]
    pub fn get_info_face_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFaceIconNameDarkness", args = 0)]
    pub fn get_face_icon_name_darkness(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFaceIconNameForHero", args = 0)]
    pub fn get_face_icon_name_for_hero(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFaceIconNameForHeroine", args = 0)]
    pub fn get_face_icon_name_for_heroine(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRingName", args = 0)]
    pub fn get_ring_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_IsDarkness", args = 0)]
    pub fn get_is_darkness(self) -> bool;

    #[method(name = "set_IsDarkness", args = 1)]
    pub fn set_is_darkness(self, value: bool) -> ();

    #[method(name = "get_IsDarknessIcon", args = 0)]
    pub fn get_is_darkness_icon(self) -> bool;

    #[method(name = "get_IsDarknessGauge", args = 0)]
    pub fn get_is_darkness_gauge(self) -> bool;

    #[method(name = "get_IsOnlyEngageWeapon", args = 0)]
    pub fn get_is_only_engage_weapon(self) -> bool;

    #[method(name = "GetGender", args = 0)]
    pub fn get_gender(self) -> crate::app::gender::Gender;

    #[method(name = "IsFemale", args = 0)]
    pub fn is_female(self) -> bool;

    #[method(name = "ChangeMain", args = 0)]
    pub fn change_main(self) -> bool;

    #[method(name = "CanSwap", args = 0)]
    pub fn can_swap(self) -> bool;

    #[method(name = "Swap", args = 0)]
    pub fn swap(self) -> bool;

    #[method(name = "Change", args = 1)]
    pub fn change(self, index: i32) -> bool;

    #[method(name = "Change", args = 1)]
    pub fn change_2(self, data: crate::app::goddata::GodData) -> bool;

    #[method(name = "SetGodDataForRewind", args = 1)]
    pub fn set_god_data_for_rewind(self, data: crate::app::goddata::GodData) -> ();

    #[method(name = "get_IsEngaging", args = 0)]
    pub fn get_is_engaging(self) -> bool;

    #[method(name = "get_IsChanged", args = 0)]
    pub fn get_is_changed(self) -> bool;

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_EngageLimit", args = 0)]
    pub fn get_engage_limit(self) -> i32;

    #[method(name = "get_Parent", args = 0)]
    pub fn get_parent(self) -> crate::app::unit::Unit;

    #[method(name = "get_Child", args = 0)]
    pub fn get_child(self) -> crate::app::unit::Unit;

    #[method(name = "SetParent", args = 2)]
    pub fn set_parent(
        self,
        unit: crate::app::unit::Unit,
        god_state: crate::app::godstate::GodState,
    ) -> ();

    #[method(name = "ClearParent", args = 0)]
    pub fn clear_parent(self) -> ();

    #[method(name = "SetChild", args = 1)]
    pub fn set_child(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearChild", args = 0)]
    pub fn clear_child(self) -> ();

    #[method(name = "GetLink", args = 1)]
    pub fn get_link(self, unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "SetTemporaryParent", args = 1)]
    pub fn set_temporary_parent(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ClearTemporaryParent", args = 1)]
    pub fn clear_temporary_parent(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "get_SynchroCounts", args = 0)]
    pub fn get_synchro_counts(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<::unity2::Il2CppString, i32>;

    #[method(name = "AddSynchroCount", args = 1)]
    pub fn add_synchro_count(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetSynchroCount", args = 2)]
    pub fn set_synchro_count(self, pid: ::unity2::Il2CppString, count: i32) -> ();

    #[method(name = "GetSynchroCount", args = 1)]
    pub fn get_synchro_count(self, pid: ::unity2::Il2CppString) -> i32;

    #[method(name = "DbgSetExp", args = 2)]
    pub fn dbg_set_exp(self, unit: crate::app::unit::Unit, exp: i32) -> ();

    #[method(name = "SetExpForRewind", args = 2)]
    pub fn set_exp_for_rewind(self, unit: crate::app::unit::Unit, exp: i32) -> ();

    #[method(name = "CanAddExp", args = 1)]
    pub fn can_add_exp(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAddExp", args = 0)]
    pub fn can_add_exp_2(self) -> bool;

    #[method(name = "TryAddExp", args = 2)]
    pub fn try_add_exp(self, unit: crate::app::unit::Unit, exp: i32) -> bool;

    #[method(name = "TryAddExpCurrentLevel", args = 2)]
    pub fn try_add_exp_current_level(self, unit: crate::app::unit::Unit, exp: i32) -> bool;

    #[method(name = "GetExp", args = 1)]
    pub fn get_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetExp", args = 0)]
    pub fn get_exp_2(self) -> i32;

    #[method(name = "GetNextLevelExp", args = 1)]
    pub fn get_next_level_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetNextLevelExp", args = 0)]
    pub fn get_next_level_exp_2(self) -> i32;

    #[method(name = "GetNextLevelCap", args = 0)]
    pub fn get_next_level_cap(self) -> i32;

    #[method(name = "GetNextLevelCap", args = 1)]
    pub fn get_next_level_cap_2(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetLevelExp", args = 1)]
    pub fn get_level_exp(self, level: i32) -> i32;

    #[method(name = "GetLevelExp", args = 2)]
    pub fn get_level_exp_2(self, unit: crate::app::unit::Unit, level: i32) -> i32;

    #[method(name = "GetMaxExp", args = 1)]
    pub fn get_max_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetMaxExp", args = 0)]
    pub fn get_max_exp_2(self) -> i32;

    #[method(name = "GetDiffExp", args = 1)]
    pub fn get_diff_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetDiffExp", args = 0)]
    pub fn get_diff_exp_2(self) -> i32;

    #[method(name = "GetMaxDiffExp", args = 1)]
    pub fn get_max_diff_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetMaxDiffExp", args = 0)]
    pub fn get_max_diff_exp_2(self) -> i32;

    #[method(name = "SetLevel", args = 2)]
    pub fn set_level(self, unit: crate::app::unit::Unit, level: i32) -> ();

    #[method(name = "SetLevel", args = 1)]
    pub fn set_level_2(self, level: i32) -> ();

    #[method(name = "SetLevelFromUnitReliance", args = 2)]
    pub fn set_level_from_unit_reliance(
        self,
        unit: crate::app::unit::Unit,
        unit_reliance_level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "DbgSetLevel", args = 2)]
    pub fn dbg_set_level(self, unit: crate::app::unit::Unit, level: i32) -> ();

    #[method(name = "GetLevel", args = 1)]
    pub fn get_level(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetLevel", args = 0)]
    pub fn get_level_2(self) -> i32;

    #[method(name = "GetLevelFromExp", args = 1)]
    pub fn get_level_from_exp(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetMaxLevel", args = 1)]
    pub fn get_max_level(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "LevelUp", args = 1)]
    pub fn level_up(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsUnlockedLevelCap", args = 0)]
    pub fn is_unlocked_level_cap(self) -> bool;

    #[method(name = "IsUnlockedLevelCap", args = 1)]
    pub fn is_unlocked_level_cap_2(god_data: crate::app::goddata::GodData) -> bool;

    #[method(name = "UnlockLevelCap", args = 0)]
    pub fn unlock_level_cap(self) -> ();

    #[method(name = "UnlockLevelCap", args = 1)]
    pub fn unlock_level_cap_2(god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "LockLevelCap", args = 0)]
    pub fn lock_level_cap(self) -> ();

    #[method(name = "LockLevelCap", args = 1)]
    pub fn lock_level_cap_2(god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "IsLevelCapNormal", args = 1)]
    pub fn is_level_cap_normal(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsLevelCapTalk", args = 1)]
    pub fn is_level_cap_talk(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsNotifiedLevelCapTalk", args = 1)]
    pub fn is_notified_level_cap_talk(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetNotifiedLevelCapTalk", args = 2)]
    pub fn set_notified_level_cap_talk(self, unit: crate::app::unit::Unit, flag: bool) -> ();

    #[method(name = "GetRelianceLevel", args = 1)]
    pub fn get_reliance_level(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::goddata::GodData_RelianceLevel;

    #[method(name = "GetMaxRelianceLevel", args = 1)]
    pub fn get_max_reliance_level(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::goddata::GodData_RelianceLevel;

    #[method(name = "CanBeRelianceLevelS", args = 1)]
    pub fn can_be_reliance_level_s(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetRelianceLevelS", args = 1)]
    pub fn set_reliance_level_s(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "DbgSetRelianceLevelS", args = 1)]
    pub fn dbg_set_reliance_level_s(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetCountOfRelianceLevelA", args = 0)]
    pub fn get_count_of_reliance_level_a(self) -> i32;

    #[method(name = "UpdateState", args = 0)]
    pub fn update_state(self) -> ();

    #[method(name = "CanTalk", args = 1)]
    pub fn can_talk(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetInheritedSkills", args = 1)]
    pub fn get_inherited_skills(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::godinheritedskills::GodInheritedSkills;

    #[method(name = "CanInheritSkills", args = 1)]
    pub fn can_inherit_skills(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAddEngageTurnLimit", args = 1)]
    pub fn can_add_engage_turn_limit(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAddEngageTurnLimit", args = 0)]
    pub fn can_add_engage_turn_limit_2(self) -> bool;

    #[method(name = "CanSubEngageCountLimit", args = 1)]
    pub fn can_sub_engage_count_limit(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanSubEngageCountLimit", args = 0)]
    pub fn can_sub_engage_count_limit_2(self) -> bool;

    #[method(name = "GetEngageAttack", args = 0)]
    pub fn get_engage_attack(self) -> crate::app::skilldata::SkillData;

    #[method(name = "IsAround", args = 2)]
    pub fn is_around(unit_a: crate::app::unit::Unit, unit_b: crate::app::unit::Unit) -> bool;

    #[method(name = "IsAround", args = 2)]
    pub fn is_around_2(
        god_unit: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "GetLinkGodUnit", args = 0)]
    pub fn get_link_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "GetEngageAttack", args = 1)]
    pub fn get_engage_attack_2(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "GetEngageSkills", args = 1)]
    pub fn get_engage_skills(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetEngageSkills", args = 0)]
    pub fn get_engage_skills_2(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetEngageSkill", args = 2)]
    pub fn get_engage_skill(
        self,
        unit: crate::app::unit::Unit,
        index: i32,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "GetEngageSkill", args = 1)]
    pub fn get_engage_skill_2(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "GetEngageItems", args = 1)]
    pub fn get_engage_items(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "GetEngageItems", args = 0)]
    pub fn get_engage_items_2(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "GetSyncroSkills", args = 1)]
    pub fn get_syncro_skills(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetSyncroSkills", args = 0)]
    pub fn get_syncro_skills_2(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetAptitude", args = 1)]
    pub fn get_aptitude(self, unit: crate::app::unit::Unit) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "GetAptitude", args = 0)]
    pub fn get_aptitude_2(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "CanUpdateForGodState", args = 0)]
    pub fn can_update_for_god_state(self) -> bool;

    #[method(name = "UpdateAllBondsForGodState", args = 1)]
    pub fn update_all_bonds_for_god_state(self, god_state: crate::app::godstate::GodState) -> ();

    #[method(name = "UpdateForGodState", args = 2)]
    pub fn update_for_god_state(
        self,
        unit: crate::app::unit::Unit,
        god_state: crate::app::godstate::GodState,
    ) -> ();

    #[method(name = "DeleteBondsExluding", args = 1)]
    pub fn delete_bonds_exluding(
        self,
        pids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "get_GodBonds", args = 0)]
    pub fn get_god_bonds(self) -> crate::app::godbondholder::GodBondHolder;

    #[method(name = "GetBond", args = 1)]
    pub fn get_bond(self, unit: crate::app::unit::Unit) -> crate::app::godbond::GodBond;

    #[method(name = "CreateBond", args = 1)]
    pub fn create_bond(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CanLink", args = 0)]
    pub fn can_link(self) -> bool;

    #[method(name = "CanChange", args = 0)]
    pub fn can_change(self) -> bool;

    #[method(name = "GetRingPrefabPath", args = 0)]
    pub fn get_ring_prefab_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDirtyLevel", args = 0)]
    pub fn get_dirty_level(self) -> i32;

    #[method(name = "get_RingDirtySep1", args = 0)]
    pub fn get_ring_dirty_sep1() -> f32;

    #[method(name = "get_RingDirtySep2", args = 0)]
    pub fn get_ring_dirty_sep2() -> f32;

    #[method(name = "AddDirty", args = 1)]
    pub fn add_dirty(self, dirty: i32) -> ();

    #[method(name = "ChangeOpponent", args = 0)]
    pub fn change_opponent(self) -> ();

    #[method(name = "InitGodWeaponRefine", args = 0)]
    pub fn init_god_weapon_refine(self) -> ();

    #[method(name = "GetGodWeaponList", args = 0)]
    pub fn get_god_weapon_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "GetGodWeaponRefineLevel", args = 2)]
    pub fn get_god_weapon_refine_level(
        self,
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> i32;

    #[method(name = "AddGodWeaponRefineEnhance", args = 2)]
    pub fn add_god_weapon_refine_enhance(
        self,
        iid: ::unity2::Il2CppString,
        capability: crate::app::capabilitysbyte::CapabilitySbyte,
    ) -> ();

    #[method(name = "GetGodWeaponRefineSkill", args = 1)]
    pub fn get_god_weapon_refine_skill(self, iid: ::unity2::Il2CppString)
        -> ::unity2::Il2CppString;

    #[method(name = "SetGodWeaponRefineLevel", args = 3)]
    pub fn set_god_weapon_refine_level(
        self,
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        level: i32,
    ) -> ();

    #[method(name = "ResetGodWeaponRefineLevel", args = 2)]
    pub fn reset_god_weapon_refine_level(
        self,
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ();

    #[method(name = "IncGodWeaponRefineLevel", args = 2)]
    pub fn inc_god_weapon_refine_level(
        self,
        iid: ::unity2::Il2CppString,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    ) -> ();

    #[method(name = "SetGodWeaponRefineSkill", args = 2)]
    pub fn set_god_weapon_refine_skill(
        self,
        iid: ::unity2::Il2CppString,
        sid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ResetGodWeaponRefineLevel", args = 1)]
    pub fn reset_god_weapon_refine_level_2(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "GetUsingCapacity", args = 1)]
    pub fn get_using_capacity(self, iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "SetUsingCapacity", args = 2)]
    pub fn set_using_capacity(self, iid: ::unity2::Il2CppString, capacity: i32) -> ();

    #[method(name = "AddUsingCapacity", args = 2)]
    pub fn add_using_capacity(self, iid: ::unity2::Il2CppString, capacity: i32) -> ();

    #[method(name = "TryGetGodWeaponRefineResult", args = 1)]
    pub fn try_get_god_weapon_refine_result(
        self,
        iid: ::unity2::Il2CppString,
    ) -> crate::app::godweaponrefineresult::GodWeaponRefineResult;

    #[method(name = "TryGetGodWeaponRefineResultEnhance", args = 1)]
    pub fn try_get_god_weapon_refine_result_enhance(
        self,
        iid: ::unity2::Il2CppString,
    ) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "TryGetGodWeaponRefineResultEquipSkills", args = 2)]
    pub fn try_get_god_weapon_refine_result_equip_skills(
        self,
        iid: ::unity2::Il2CppString,
        is_enchant: bool,
    ) -> crate::app::skillarray::SkillArray;

    #[method(name = "CopyGodWeaponRefineFrom", args = 1)]
    pub fn copy_god_weapon_refine_from(self, from: crate::app::godunit::GodUnit) -> ();

    #[method(name = "CorrectBond", args = 1)]
    pub fn correct_bond(self, instance_id: i32) -> ();

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_SortKey", args = 0)]
    pub fn get_sort_key(self) -> i32;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(p: crate::app::godunit::GodUnit) -> crate::app::goddata::GodData;

    #[method(name = "ChangeCancel", args = 0)]
    pub fn change_cancel(self) -> ();

    #[method(name = "ChangeResume", args = 0)]
    pub fn change_resume(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-godunit")]
impl GodUnit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodUnit),
                ::core::stringify!(new),
            )
        });
        <Self as IGodUnitMethods>::ctor(this);
        this
    }
}
