
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleinfoside/BattleInfoSide_BitFieldStatus.md")))]
#[::unity2::class(namespace = "App", name = "BattleInfoSide.BitFieldStatus")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: battleinfoside :: BattleInfoSide_Status >)]
pub struct BattleInfoSide_BitFieldStatus {}

#[cfg(feature = "app-battleinfoside")]
#[::unity2::methods]
impl BattleInfoSide_BitFieldStatus {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::battleinfoside::BattleInfoSide_Status) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battleinfoside")]
impl BattleInfoSide_BitFieldStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleInfoSide_BitFieldStatus),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleInfoSide_BitFieldStatusMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleinfoside/BattleInfoSide_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleInfoSide_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleInfoSide_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleInfoSide.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleInfoSide_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleInfoSide_Status {
    pub fn offense() -> Self {
        Self { value: 1 }
    }

    pub fn defense() -> Self {
        Self { value: 2 }
    }

    pub fn chain_attack() -> Self {
        Self { value: 4 }
    }

    pub fn chain_guard() -> Self {
        Self { value: 8 }
    }

    pub fn engage_link() -> Self {
        Self { value: 16 }
    }

    pub fn ignore_position() -> Self {
        Self { value: 32 }
    }

    pub fn ignore_range() -> Self {
        Self { value: 64 }
    }

    pub fn magic() -> Self {
        Self { value: 128 }
    }

    pub fn rod() -> Self {
        Self { value: 256 }
    }

    pub fn heal_rod() -> Self {
        Self { value: 512 }
    }

    pub fn interference_rod() -> Self {
        Self { value: 1024 }
    }

    pub fn long_range() -> Self {
        Self { value: 2048 }
    }

    pub fn not_weapon() -> Self {
        Self { value: 4096 }
    }

    pub fn not_attack() -> Self {
        Self { value: 8192 }
    }

    pub fn not_stun() -> Self {
        Self { value: 16384 }
    }

    pub fn move_chain_attack() -> Self {
        Self { value: 32768 }
    }

    pub fn haunt_chain_attack() -> Self {
        Self { value: 65536 }
    }

    pub fn exp_battle() -> Self {
        Self { value: 131072 }
    }

    pub fn exp_destroy() -> Self {
        Self { value: 262144 }
    }

    pub fn exp_rod() -> Self {
        Self { value: 524288 }
    }

    pub fn exp_rod_miss() -> Self {
        Self { value: 1048576 }
    }

    pub fn give_exp_battle() -> Self {
        Self { value: 2097152 }
    }

    pub fn gained() -> Self {
        Self { value: 8388608 }
    }

    pub fn dead() -> Self {
        Self { value: 16777216 }
    }

    pub fn chain_attacked() -> Self {
        Self { value: 33554432 }
    }

    pub fn chain_guarded() -> Self {
        Self { value: 67108864 }
    }

    pub fn blown() -> Self {
        Self { value: 134217728 }
    }

    pub fn bounced() -> Self {
        Self { value: 268435456 }
    }

    pub fn breaked() -> Self {
        Self { value: 536870912 }
    }

    pub fn interrupting() -> Self {
        Self { value: 1073741824 }
    }

    pub fn change_dragon() -> Self {
        Self { value: -2147483648 }
    }

    pub fn mask_no_attack() -> Self {
        Self { value: 12544 }
    }

    pub fn mask_exp() -> Self {
        Self { value: 1966080 }
    }

    pub fn mask_chain() -> Self {
        Self { value: 12 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleinfoside/BattleInfoSide.md")))]
#[::unity2::class(namespace = "App", name = "BattleInfoSide")]
#[parent(crate::system::object::Object)]
pub struct BattleInfoSide {
    #[rename(name = "m_Info")]
    pub m_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_SideType")]
    pub m_side_type: crate::app::battleside::BattleSide_Type,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_SpecifiedItem")]
    pub m_specified_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_Terrain")]
    pub m_terrain: crate::app::terraindata_2::TerrainData_2,
    #[rename(name = "m_Overlap")]
    pub m_overlap: crate::app::terraindata_2::TerrainData_2,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::battleinfoside::BattleInfoSide_BitFieldStatus,
    #[rename(name = "m_Detail")]
    pub m_detail: crate::app::battledetail::BattleDetail,
    #[rename(name = "m_Hierarchy")]
    pub m_hierarchy: crate::app::pool::Pool_Hierarchy_1<crate::app::battledetail::BattleDetail>,
    #[rename(name = "m_Support")]
    pub m_support: crate::app::supportcalculator::SupportCalculator,
    #[rename(name = "m_Parent")]
    pub m_parent: crate::app::battleinfoside::BattleInfoSide,
    #[rename(name = "m_Reverse")]
    pub m_reverse: crate::app::battleinfoside::BattleInfoSide,
    #[rename(name = "m_Destroy")]
    pub m_destroy: crate::app::battledestory::BattleDestory,
    #[rename(name = "m_MaskSkill")]
    pub m_mask_skill: crate::app::skillarray::SkillArray,
    #[static_field]
    #[rename(name = "ContinueCondition")]
    pub continue_condition: i32,
}

#[cfg(feature = "app-battleinfoside")]
#[::unity2::methods]
impl BattleInfoSide {
    #[method(name = "get_DebuggerDisplay", args = 0)]
    pub fn get_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        info: crate::app::battleinfo::BattleInfo,
        side: crate::app::battleside::BattleSide_Type,
    ) -> ();

    #[method(name = "get_Exist", args = 0)]
    pub fn get_exist(self) -> bool;

    #[method(name = "get_SideType", args = 0)]
    pub fn get_side_type(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "get_Info", args = 0)]
    pub fn get_info(self) -> crate::app::battleinfo::BattleInfo;

    #[method(name = "get_Parent", args = 0)]
    pub fn get_parent(self) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "set_Parent", args = 1)]
    pub fn set_parent(self, value: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "get_Reverse", args = 0)]
    pub fn get_reverse(self) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "set_Reverse", args = 1)]
    pub fn set_reverse(self, value: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "get_MaskSkill", args = 0)]
    pub fn get_mask_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PrefixlessPid", args = 0)]
    pub fn get_prefixless_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_Hp", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "set_Hp", args = 1)]
    pub fn set_hp(self, value: i32) -> ();

    #[method(name = "get_GainExp", args = 0)]
    pub fn get_gain_exp(self) -> i32;

    #[method(name = "set_GainExp", args = 1)]
    pub fn set_gain_exp(self, value: i32) -> ();

    #[method(name = "get_GainGold", args = 0)]
    pub fn get_gain_gold(self) -> i32;

    #[method(name = "set_GainGold", args = 1)]
    pub fn set_gain_gold(self, value: i32) -> ();

    #[method(name = "get_DropItemRatio", args = 0)]
    pub fn get_drop_item_ratio(self) -> f32;

    #[method(name = "set_DropItemRatio", args = 1)]
    pub fn set_drop_item_ratio(self, value: f32) -> ();

    #[method(name = "get_PickupItemIndex", args = 0)]
    pub fn get_pickup_item_index(self) -> i32;

    #[method(name = "set_PickupItemIndex", args = 1)]
    pub fn set_pickup_item_index(self, value: i32) -> ();

    #[method(name = "get_Damage", args = 0)]
    pub fn get_damage(self) -> i32;

    #[method(name = "set_Damage", args = 1)]
    pub fn set_damage(self, value: i32) -> ();

    #[method(name = "get_Heal", args = 0)]
    pub fn get_heal(self) -> i32;

    #[method(name = "set_Heal", args = 1)]
    pub fn set_heal(self, value: i32) -> ();

    #[method(name = "get_BattleTimes", args = 0)]
    pub fn get_battle_times(self) -> i32;

    #[method(name = "set_BattleTimes", args = 1)]
    pub fn set_battle_times(self, value: i32) -> ();

    #[method(name = "get_TotalOrder", args = 0)]
    pub fn get_total_order(self) -> i32;

    #[method(name = "set_TotalOrder", args = 1)]
    pub fn set_total_order(self, value: i32) -> ();

    #[method(name = "get_TotalAction", args = 0)]
    pub fn get_total_action(self) -> i32;

    #[method(name = "set_TotalAction", args = 1)]
    pub fn set_total_action(self, value: i32) -> ();

    #[method(name = "get_TotalAttack", args = 0)]
    pub fn get_total_attack(self) -> i32;

    #[method(name = "set_TotalAttack", args = 1)]
    pub fn set_total_attack(self, value: i32) -> ();

    #[method(name = "get_TotalDamage", args = 0)]
    pub fn get_total_damage(self) -> i32;

    #[method(name = "set_TotalDamage", args = 1)]
    pub fn set_total_damage(self, value: i32) -> ();

    #[method(name = "get_TotalResult", args = 0)]
    pub fn get_total_result(self) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "set_TotalResult", args = 1)]
    pub fn set_total_result(self, value: crate::app::battlescene::BattleScene_Result) -> ();

    #[method(name = "get_Temporary", args = 0)]
    pub fn get_temporary(self) -> i32;

    #[method(name = "set_Temporary", args = 1)]
    pub fn set_temporary(self, value: i32) -> ();

    #[method(name = "get_Stun", args = 0)]
    pub fn get_stun(self) -> i32;

    #[method(name = "set_Stun", args = 1)]
    pub fn set_stun(self, value: i32) -> ();

    #[method(name = "get_EngageCount", args = 0)]
    pub fn get_engage_count(self) -> i32;

    #[method(name = "set_EngageCount", args = 1)]
    pub fn set_engage_count(self, value: i32) -> ();

    #[method(name = "get_EngageFirstCount", args = 0)]
    pub fn get_engage_first_count(self) -> i32;

    #[method(name = "set_EngageFirstCount", args = 1)]
    pub fn set_engage_first_count(self, value: i32) -> ();

    #[method(name = "get_BlownDistance", args = 0)]
    pub fn get_blown_distance(self) -> i32;

    #[method(name = "set_BlownDistance", args = 1)]
    pub fn set_blown_distance(self, value: i32) -> ();

    #[method(name = "get_WeaponExpend", args = 0)]
    pub fn get_weapon_expend(self) -> i32;

    #[method(name = "set_WeaponExpend", args = 1)]
    pub fn set_weapon_expend(self, value: i32) -> ();

    #[method(name = "get_ExpendCount", args = 0)]
    pub fn get_expend_count(self) -> i32;

    #[method(name = "set_ExpendCount", args = 1)]
    pub fn set_expend_count(self, value: i32) -> ();

    #[method(name = "PushDetail", args = 0)]
    pub fn push_detail(self) -> ();

    #[method(name = "PopDetail", args = 0)]
    pub fn pop_detail(self) -> ();

    #[method(name = "GetEngageCountLimit", args = 0)]
    pub fn get_engage_count_limit(self) -> i32;

    #[method(name = "GetCenterPosition", args = 0)]
    pub fn get_center_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetEffectPosition", args = 0)]
    pub fn get_effect_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TryFocus", args = 0)]
    pub fn try_focus(self) -> bool;

    #[method(name = "Focus", args = 0)]
    pub fn focus(self) -> ();

    #[method(name = "ClampHP", args = 1)]
    pub fn clamp_hp(self, value: i32) -> i32;

    #[method(name = "CommitHp", args = 0)]
    pub fn commit_hp(self) -> i32;

    #[method(name = "CommitHp", args = 2)]
    pub fn commit_hp_2(self, damage: i32, heal: i32) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "IsChangeDragonImpl", args = 0)]
    pub fn is_change_dragon_impl(self) -> bool;

    #[method(name = "ComplementConditions", args = 1)]
    pub fn complement_conditions(self, range: i32) -> ();

    #[method(name = "ComplementDragon", args = 0)]
    pub fn complement_dragon(self) -> ();

    #[method(name = "CalcSupport", args = 0)]
    pub fn calc_support(self) -> ();

    #[method(name = "CalcCapability", args = 0)]
    pub fn calc_capability(self) -> ();

    #[method(name = "CalcDetail", args = 0)]
    pub fn calc_detail(self) -> ();

    #[method(name = "CalcInterferenceRod", args = 0)]
    pub fn calc_interference_rod(self) -> ();

    #[method(name = "CalcBattleTimesImpl", args = 1)]
    pub fn calc_battle_times_impl(self, flag: crate::app::battleinfo::BattleInfo_FlagField) -> i32;

    #[method(name = "CalcBattleTimes", args = 1)]
    pub fn calc_battle_times(self, flag: crate::app::battleinfo::BattleInfo_FlagField) -> ();

    #[method(name = "GetAttack", args = 0)]
    pub fn get_attack(self) -> i32;

    #[method(name = "GetDefense", args = 0)]
    pub fn get_defense(self) -> i32;

    #[method(name = "GetHit", args = 0)]
    pub fn get_hit(self) -> i32;

    #[method(name = "GetCritical", args = 0)]
    pub fn get_critical(self) -> i32;

    #[method(name = "GetAvoid", args = 0)]
    pub fn get_avoid(self) -> i32;

    #[method(name = "GetSecure", args = 0)]
    pub fn get_secure(self) -> i32;

    #[method(name = "GetContinuous", args = 0)]
    pub fn get_continuous(self) -> i32;

    #[method(name = "GetSimplePower", args = 1)]
    pub fn get_simple_power(self, is_critical: bool) -> i32;

    #[method(name = "GetSimpleHit", args = 0)]
    pub fn get_simple_hit(self) -> i32;

    #[method(name = "GetSimpleCritical", args = 0)]
    pub fn get_simple_critical(self) -> i32;

    #[method(name = "GetBlowRatio", args = 0)]
    pub fn get_blow_ratio(self) -> i32;

    #[method(name = "GetBlowDistance", args = 0)]
    pub fn get_blow_distance(self) -> i32;

    #[method(name = "GetActionCount", args = 0)]
    pub fn get_action_count(self) -> i32;

    #[method(name = "GetAttackCount", args = 0)]
    pub fn get_attack_count(self) -> i32;

    #[method(name = "GetTotalAttackCount", args = 0)]
    pub fn get_total_attack_count(self) -> i32;

    #[method(name = "CanBreakable", args = 0)]
    pub fn can_breakable(self) -> bool;

    #[method(name = "CanSkyBattle", args = 0)]
    pub fn can_sky_battle(self) -> bool;

    #[method(name = "CanRevive", args = 0)]
    pub fn can_revive(self) -> bool;

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnit", args = 3)]
    pub fn set_unit_2(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "SetUnit", args = 1)]
    pub fn set_unit_3(self, src: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "SetUnitItem", args = 1)]
    pub fn set_unit_item(self, item_index: i32) -> ();

    #[method(name = "SetUnitItem", args = 1)]
    pub fn set_unit_item_2(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "SetUnitItem", args = 1)]
    pub fn set_unit_item_3(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetUnitSkill", args = 1)]
    pub fn set_unit_skill(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetSpecifiedItem", args = 1)]
    pub fn set_specified_item(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "SetTerrain", args = 2)]
    pub fn set_terrain(self, x: i32, z: i32) -> ();

    #[method(name = "GetStatus", args = 0)]
    pub fn get_status(self) -> crate::app::battleinfoside::BattleInfoSide_BitFieldStatus;

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_BmapSize", args = 0)]
    pub fn get_bmap_size(self) -> i32;

    #[method(name = "get_UnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "get_Actor", args = 0)]
    pub fn get_actor(self) -> crate::app::unitactor::UnitActor;

    #[method(name = "get_Destroy", args = 0)]
    pub fn get_destroy(self) -> crate::app::battledestory::BattleDestory;

    #[method(name = "get_GodUnit", args = 0)]
    pub fn get_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Terrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "get_Overlap", args = 0)]
    pub fn get_overlap(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "get_Detail", args = 0)]
    pub fn get_detail(self) -> crate::app::battledetail::BattleDetail;

    #[method(name = "get_Support", args = 0)]
    pub fn get_support(self) -> crate::app::supportcalculator::SupportCalculator;

    #[method(name = "get_NowHp", args = 0)]
    pub fn get_now_hp(self) -> i32;

    #[method(name = "get_MaxHp", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "GetActualUnitItem", args = 0)]
    pub fn get_actual_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetMaxHeal", args = 0)]
    pub fn get_max_heal(self) -> i32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(self, name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float_2(self, command: crate::app::calculatorcommand::CalculatorCommand) -> f32;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int_2(self, command: crate::app::calculatorcommand::CalculatorCommand) -> i32;

    #[method(name = "GetParticipationUnit", args = 0)]
    pub fn get_participation_unit(self) -> crate::app::unit::Unit;

    #[method(name = "IsStandingDie", args = 0)]
    pub fn is_standing_die(self) -> bool;

    #[method(name = "IsImmortal", args = 0)]
    pub fn is_immortal(self) -> bool;

    #[method(name = "IsLastBoss", args = 0)]
    pub fn is_last_boss(self) -> bool;

    #[method(name = "IsChangeDragon", args = 0)]
    pub fn is_change_dragon(self) -> bool;

    #[method(name = "CalcActiveSkill", args = 2)]
    pub fn calc_active_skill(
        self,
        timing: crate::app::skilldata::SkillData_Timings,
        action: crate::app::skilldata::SkillData_Actions,
    ) -> ();

    #[method(name = "IsIgnoreSkill", args = 1)]
    pub fn is_ignore_skill(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "IsIgnoreSkillImpl", args = 1)]
    pub fn is_ignore_skill_impl(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "IsEnableSkill", args = 2)]
    pub fn is_enable_skill(
        self,
        skill: crate::app::skilldata::SkillData,
        action: crate::app::skilldata::SkillData_Actions,
    ) -> bool;

    #[method(name = "CalcActiveSkill", args = 2)]
    pub fn calc_active_skill_2(
        self,
        skill: crate::app::skilldata::SkillData,
        action: crate::app::skilldata::SkillData_Actions,
    ) -> bool;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleside::BattleSide_Type;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-battleinfoside")]
impl BattleInfoSide {
    pub fn new(
        info: crate::app::battleinfo::BattleInfo,
        side: crate::app::battleside::BattleSide_Type,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleInfoSide),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleInfoSideMethods>::ctor(this, info, side);
        this
    }
}
