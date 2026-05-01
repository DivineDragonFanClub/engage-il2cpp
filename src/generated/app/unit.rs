
use crate::app::bitfield64::BitField64;
use crate::app::bitfield64::IBitField64;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate64_1::BitFieldTemplate64_1;
use crate::app::bitfieldtemplate64_1::IBitFieldTemplate64_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_StatusField.md")))]
#[::unity2::class(namespace = "App", name = "Unit.StatusField")]
# [parent (crate :: app :: bitfieldtemplate64_1 :: BitFieldTemplate64_1 < crate :: app :: unit :: Unit_Status >)]
pub struct Unit_StatusField {}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_StatusField {
    #[method(name = "ToLong", args = 1)]
    pub fn to_long(self, value: crate::app::unit::Unit_Status) -> i64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_StatusField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_StatusField),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_StatusFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_ItemsForSelectedWeapon.md")))]
#[::unity2::class(namespace = "App", name = "Unit.ItemsForSelectedWeapon")]
#[parent(crate::system::object::Object)]
pub struct Unit_ItemsForSelectedWeapon {
    #[rename(name = "m_Items")]
    pub m_items: ::unity2::Array<crate::app::itemdata::ItemData>,
}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_ItemsForSelectedWeapon {
    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "GetCount", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Get", args = 1)]
    pub fn get(self, index: i32) -> crate::app::itemdata::ItemData;

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_ItemsForSelectedWeapon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_ItemsForSelectedWeapon),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_ItemsForSelectedWeaponMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_CalcInfo.md")))]
#[::unity2::class(namespace = "App", name = "Unit.CalcInfo")]
#[parent(crate::system::object::Object)]
pub struct Unit_CalcInfo {
    #[rename(name = "Count")]
    pub count: i32,
    #[rename(name = "Attack")]
    pub attack: i32,
    #[rename(name = "Hit")]
    pub hit: i32,
    #[rename(name = "Avoid")]
    pub avoid: i32,
    #[rename(name = "Critical")]
    pub critical: i32,
    #[rename(name = "Secure")]
    pub secure: i32,
    #[rename(name = "Continuous")]
    pub continuous: i32,
    #[rename(name = "PhysicalAttack")]
    pub physical_attack: i32,
    #[rename(name = "MagicAttack")]
    pub magic_attack: i32,
    #[rename(name = "PhysicalDefense")]
    pub physical_defense: i32,
    #[rename(name = "MagicDefense")]
    pub magic_defense: i32,
}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_CalcInfo {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, unit: crate::app::unit::Unit) -> crate::app::unit::Unit_CalcInfo;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_CalcInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_CalcInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_CalcInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit.md")))]
#[::unity2::class(namespace = "App", name = "Unit")]
#[parent(crate::system::object::Object)]
pub struct Unit {
    #[static_field]
    #[rename(name = "ItemMax")]
    pub item_max: i32,
    #[static_field]
    #[rename(name = "ExpMax")]
    pub exp_max: i32,
    #[static_field]
    #[rename(name = "EnhanceMoveMax")]
    pub enhance_move_max: i32,
    #[static_field]
    #[rename(name = "StunMax")]
    pub stun_max: i32,
    #[static_field]
    #[rename(name = "EquipSkillMax")]
    pub equip_skill_max: i32,
    #[static_field]
    #[rename(name = "InternalLevelMin")]
    pub internal_level_min: i32,
    #[static_field]
    #[rename(name = "InternalLevelMax")]
    pub internal_level_max: i32,
    #[static_field]
    #[rename(name = "SkillPointMax")]
    pub skill_point_max: i32,
    #[static_field]
    #[rename(name = "CellCountMax")]
    pub cell_count_max: i32,
    #[static_field]
    #[rename(name = "LevelUpRetryMax")]
    pub level_up_retry_max: i32,
    #[static_field]
    #[rename(name = "DefaultGodStates")]
    pub default_god_states: ::unity2::Array<crate::app::godstate::GodState>,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::unit::Unit_StatusField,
    #[rename(name = "m_Prev")]
    pub m_prev: crate::app::unit::Unit,
    #[rename(name = "m_Next")]
    pub m_next: crate::app::unit::Unit,
    #[rename(name = "m_Ai")]
    pub m_ai: crate::app::unitai::UnitAI,
    #[rename(name = "m_Edit")]
    pub m_edit: crate::app::unitedit::UnitEdit,
    #[rename(name = "m_Ident")]
    pub m_ident: i32,
    #[rename(name = "m_Person")]
    pub m_person: crate::app::persondata::PersonData,
    #[rename(name = "m_Job")]
    pub m_job: crate::app::jobdata::JobData,
    #[rename(name = "m_Force")]
    pub m_force: crate::app::force::Force,
    #[rename(name = "m_BaseCapability")]
    pub m_base_capability: crate::app::unitbasecapability::UnitBaseCapability,
    #[rename(name = "m_GrowCapability")]
    pub m_grow_capability: crate::app::capability::Capability,
    #[rename(name = "m_LevelCapability")]
    pub m_level_capability: crate::app::unitbasecapability::UnitBaseCapability,
    #[rename(name = "m_GrowSeed")]
    pub m_grow_seed: u32,
    #[rename(name = "m_DropSeed")]
    pub m_drop_seed: u32,
    #[rename(name = "m_Actor")]
    pub m_actor: crate::app::unitactor::UnitActor,
    #[rename(name = "m_Info")]
    pub m_info: crate::app::mapinforoot::MapInfoRoot,
    #[rename(name = "m_Index")]
    pub m_index: u8,
    #[rename(name = "m_Level")]
    pub m_level: u8,
    #[rename(name = "m_Exp")]
    pub m_exp: u8,
    #[rename(name = "m_Hp")]
    pub m_hp: crate::app::unit::Unit_ChangeValue,
    #[rename(name = "m_HpStockCount")]
    pub m_hp_stock_count: u8,
    #[rename(name = "m_HpStockCountMax")]
    pub m_hp_stock_count_max: u8,
    #[rename(name = "m_ExtraHpStockCount")]
    pub m_extra_hp_stock_count: u8,
    #[rename(name = "m_ExtraHpStockCountMax")]
    pub m_extra_hp_stock_count_max: u8,
    #[rename(name = "m_EngageCount")]
    pub m_engage_count: u8,
    #[rename(name = "m_EngageTurn")]
    pub m_engage_turn: u8,
    #[rename(name = "m_EngageCountView")]
    pub m_engage_count_view: u8,
    #[rename(name = "m_GodStates")]
    pub m_god_states: ::unity2::Array<crate::app::godstate::GodState>,
    #[rename(name = "m_X")]
    pub m_x: i8,
    #[rename(name = "m_Z")]
    pub m_z: i8,
    #[rename(name = "m_DisposX")]
    pub m_dispos_x: i8,
    #[rename(name = "m_DisposZ")]
    pub m_dispos_z: i8,
    #[rename(name = "m_Angle")]
    pub m_angle: f32,
    #[rename(name = "m_DontAttackPerson")]
    pub m_dont_attack_person: crate::app::persondata::PersonData,
    #[rename(name = "m_DontAttackForceMask")]
    pub m_dont_attack_force_mask: u32,
    #[rename(name = "m_ItemList")]
    pub m_item_list: crate::app::unititemlist::UnitItemList,
    #[rename(name = "m_ItemSelected")]
    pub m_item_selected: crate::app::unititem::UnitItem,
    #[rename(name = "m_AccessoryList")]
    pub m_accessory_list: crate::app::unitaccessorylist::UnitAccessoryList,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_GodLink")]
    pub m_god_link: crate::app::godunit::GodUnit,
    #[rename(name = "m_Ring")]
    pub m_ring: crate::app::unitring::UnitRing,
    #[rename(name = "m_ExtraSight")]
    pub m_extra_sight: i32,
    #[rename(name = "m_MoveDistance")]
    pub m_move_distance: i32,
    #[rename(name = "m_MaskSkill")]
    pub m_mask_skill: crate::app::skillarray::SkillArray,
    #[rename(name = "m_EquipSkill")]
    pub m_equip_skill: crate::app::skillarray::SkillArray,
    #[rename(name = "m_PrivateSkill")]
    pub m_private_skill: crate::app::skillarray::SkillArray,
    #[rename(name = "m_ReceiveSkill")]
    pub m_receive_skill: crate::app::skillarray::SkillArray,
    #[rename(name = "m_SupportedSkill")]
    pub m_supported_skill: crate::app::skillarray::SkillArray,
    #[rename(name = "m_EquipSkillPool")]
    pub m_equip_skill_pool: crate::app::skillarray::SkillArray,
    #[rename(name = "m_LearnedJobSkill")]
    pub m_learned_job_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_OriginalAptitude")]
    pub m_original_aptitude: crate::app::weaponmask::WeaponMask,
    #[rename(name = "m_Aptitude")]
    pub m_aptitude: crate::app::weaponmask::WeaponMask,
    #[rename(name = "m_WeaponMask")]
    pub m_weapon_mask: crate::app::weaponmask::WeaponMask,
    #[rename(name = "m_SelectedWeaponMask")]
    pub m_selected_weapon_mask: crate::app::weaponmask::WeaponMask,
    #[rename(name = "m_EnhanceFactors")]
    pub m_enhance_factors: crate::app::unitenhancefactors::UnitEnhanceFactors,
    #[rename(name = "m_EnhanceCalculator")]
    pub m_enhance_calculator: crate::app::unitenhancecalculator::UnitEnhanceCalculator,
    #[rename(name = "m_InternalLevel")]
    pub m_internal_level: i8,
    #[rename(name = "m_LastPickVoice")]
    pub m_last_pick_voice: i8,
    #[rename(name = "m_AttackImage")]
    pub m_attack_image: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[rename(name = "m_RodImage")]
    pub m_rod_image: crate::app::mapdeployrodimage::MapDeployRodImage,
    #[rename(name = "m_HealImage")]
    pub m_heal_image: crate::app::mapdeployhealimage::MapDeployHealImage,
    #[rename(name = "m_SupportImage")]
    pub m_support_image: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    #[rename(name = "m_InterferenceImage")]
    pub m_interference_image: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    #[rename(name = "m_EngageImage")]
    pub m_engage_image: crate::app::mapdeployengageimage::MapDeployEngageImage,
    #[rename(name = "m_MoveImage")]
    pub m_move_image: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[rename(name = "m_Record")]
    pub m_record: crate::app::unitrecord::UnitRecord,
    #[rename(name = "m_MapHistoryIndex")]
    pub m_map_history_index: u8,
    #[rename(name = "m_MaskSkillLock")]
    pub m_mask_skill_lock: ::unity2::IlInstance,
    #[rename(name = "m_FortuneTarget")]
    pub m_fortune_target: crate::app::persondata::PersonData,
    #[rename(name = "m_FortuneSeed")]
    pub m_fortune_seed: u32,
    #[rename(name = "m_RelayPlayerIndex")]
    pub m_relay_player_index: u8,
    #[rename(name = "m_SkillPoint")]
    pub m_skill_point: i16,
    #[rename(name = "m_OwnerUnit")]
    pub m_owner_unit: i32,
    #[rename(name = "m_LockTargetX")]
    pub m_lock_target_x: i8,
    #[rename(name = "m_LockTargetZ")]
    pub m_lock_target_z: i8,
    #[static_field]
    #[rename(name = "s_AttackImage")]
    pub s_attack_image: crate::app::mapdeployattackimage::MapDeployAttackImage,
    #[static_field]
    #[rename(name = "s_RodImage")]
    pub s_rod_image: crate::app::mapdeployrodimage::MapDeployRodImage,
    #[static_field]
    #[rename(name = "s_HealImage")]
    pub s_heal_image: crate::app::mapdeployhealimage::MapDeployHealImage,
    #[static_field]
    #[rename(name = "s_SupportImage")]
    pub s_support_image: crate::app::mapdeploysupportimage::MapDeploySupportImage,
    #[static_field]
    #[rename(name = "s_InterferenceImage")]
    pub s_interference_image: crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage,
    #[static_field]
    #[rename(name = "s_EngageImage")]
    pub s_engage_image: crate::app::mapdeployengageimage::MapDeployEngageImage,
    #[static_field]
    #[rename(name = "s_MoveImage")]
    pub s_move_image: crate::app::mapdeploymoveimage::MapDeployMoveImage,
    #[static_field]
    #[rename(name = "LeaderAddLevel")]
    pub leader_add_level: i32,
    #[static_field]
    #[rename(name = "s_Engaging")]
    pub s_engaging: bool,
    #[static_field]
    #[rename(name = "s_Mind")]
    pub s_mind: crate::app::mapmind::MapMind_Type,
    #[static_field]
    #[rename(name = "s_CanEngageStart")]
    pub s_can_engage_start: bool,
    #[static_field]
    #[rename(name = "s_UnitList")]
    pub s_unit_list: crate::app::unititemlist::UnitItemList,
    #[static_field]
    #[rename(name = "s_TempList")]
    pub s_temp_list: crate::app::unititemlist::UnitItemList,
    #[static_field]
    #[rename(name = "s_TempTarget")]
    pub s_temp_target: crate::app::maptarget::MapTarget_DataSet,
    #[static_field]
    #[rename(name = "Disorder")]
    pub disorder: crate::app::skilldata::SkillData_States,
    #[static_field]
    #[rename(name = "GrowAbortCount")]
    pub grow_abort_count: i32,
    #[static_field]
    #[rename(name = "InvalidEngageCount")]
    pub invalid_engage_count: u8,
    #[static_field]
    #[rename(name = "EnemyEngageMask")]
    pub enemy_engage_mask: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<
        crate::app::mapdnagerdeploy::MapDnagerDeploy,
    >,
    #[rename(name = "m_CalcInfo")]
    pub m_calc_info: crate::app::unit::Unit_CalcInfo,
}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, use_image: bool) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, index: i32) -> ();

    #[method(name = "Create", args = 4)]
    pub fn create(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        level: i32,
        random: crate::app::random_2::Random_2,
    ) -> ();

    #[method(name = "CreateFromDispos", args = 1)]
    pub fn create_from_dispos(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "CalcEncountLevel", args = 2)]
    pub fn calc_encount_level(dispos: crate::app::disposdata::DisposData, level: i32) -> i32;

    #[method(name = "CalcEncountJob", args = 4)]
    pub fn calc_encount_job(
        dispos: crate::app::disposdata::DisposData,
        random: crate::app::random_2::Random_2,
        level: i32,
        job: crate::app::jobdata::JobData,
    ) -> crate::app::jobdata::JobData;

    #[method(name = "CreateChallengeEnemy", args = 4)]
    pub fn create_challenge_enemy(
        self,
        dispos: crate::app::disposdata::DisposData,
        random: crate::app::random_2::Random_2,
        level: i32,
        ejid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateEncountEnemy", args = 3)]
    pub fn create_encount_enemy(
        self,
        dispos: crate::app::disposdata::DisposData,
        unit_rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
        job: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "SetEncountHpStock", args = 2)]
    pub fn set_encount_hp_stock(
        self,
        dispos: crate::app::disposdata::DisposData,
        difficulty: crate::app::difficulty::Difficulty,
    ) -> ();

    #[method(name = "SetHpStockCount", args = 1)]
    pub fn set_hp_stock_count(self, count: i32) -> ();

    #[method(name = "CreateEncountPost", args = 4)]
    pub fn create_encount_post(
        self,
        dispos: crate::app::disposdata::DisposData,
        level: i32,
        offset: bool,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> ();

    #[method(name = "CreateEncountMob", args = 3)]
    pub fn create_encount_mob(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        equip_weapon_iid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateDlcGodEnemy", args = 2)]
    pub fn create_dlc_god_enemy(
        self,
        data: crate::app::disposdata::DisposData,
        is_clear_all: bool,
    ) -> ();

    #[method(name = "CreateDlcGodEnemyImpl1", args = 3)]
    pub fn create_dlc_god_enemy_impl1(
        self,
        dispos: crate::app::disposdata::DisposData,
        level: i32,
        random: crate::app::random_2::Random_2,
    ) -> ();

    #[method(name = "CreateForVision", args = 2)]
    pub fn create_for_vision(
        self,
        original: crate::app::unit::Unit,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "CreateForSummon", args = 3)]
    pub fn create_for_summon(
        self,
        original: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "CreateForSummonTelop", args = 3)]
    pub fn create_for_summon_telop(
        self,
        original: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "CreateForSummonCommon", args = 3)]
    pub fn create_for_summon_common(
        self,
        original: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "CreateForArena", args = 3)]
    pub fn create_for_arena(
        self,
        emblem: crate::app::godunit::GodUnit,
        level: i32,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "CreateForVersus", args = 1)]
    pub fn create_for_versus(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "AutoGrowCapability", args = 2)]
    pub fn auto_grow_capability(self, level: i32, target_level: i32) -> ();

    #[method(name = "AutoGrowCapability", args = 2)]
    pub fn auto_grow_capability_2(
        self,
        level: i32,
        percents: crate::app::capabilityint::CapabilityInt,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ClearExceptMapHistoryIndex", args = 0)]
    pub fn clear_except_map_history_index(self) -> ();

    #[method(name = "ClearImpl", args = 1)]
    pub fn clear_impl(self, is_clear_map_history_index: bool) -> ();

    #[method(name = "ClearForRewindPreview", args = 1)]
    pub fn clear_for_rewind_preview(self, changed_gid: ::unity2::Il2CppString) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::unit::Unit) -> ();

    #[method(name = "CopyFromForLevelUp", args = 1)]
    pub fn copy_from_for_level_up(self, from: crate::app::unit::Unit) -> ();

    #[method(name = "CopyFromForArenaResult", args = 1)]
    pub fn copy_from_for_arena_result(self, from: crate::app::unit::Unit) -> ();

    #[method(name = "CopyFromForVersus", args = 1)]
    pub fn copy_from_for_versus(self, from: crate::app::unit::Unit) -> ();

    #[method(name = "ResetPhaseBegin", args = 1)]
    pub fn reset_phase_begin(self, phase: crate::app::force::Force_Type) -> ();

    #[method(name = "IsEngageOwner", args = 0)]
    pub fn is_engage_owner(self) -> bool;

    #[method(name = "ResetPhaseBeginAfter", args = 1)]
    pub fn reset_phase_begin_after(self, phase: crate::app::force::Force_Type) -> ();

    #[method(name = "ResetPhaseEnd", args = 1)]
    pub fn reset_phase_end(self, phase: crate::app::force::Force_Type) -> ();

    #[method(name = "ResetSubPhaseCharmConfusionBegin", args = 0)]
    pub fn reset_sub_phase_charm_confusion_begin(self) -> ();

    #[method(name = "ResetDead", args = 0)]
    pub fn reset_dead(self) -> ();

    #[method(name = "ResetMapBegin", args = 0)]
    pub fn reset_map_begin(self) -> ();

    #[method(name = "ResetMapEnd", args = 1)]
    pub fn reset_map_end(self, is_reset_position: bool) -> ();

    #[method(name = "MapCompleted", args = 0)]
    pub fn map_completed(self) -> ();

    #[method(name = "Fixed", args = 0)]
    pub fn fixed(self) -> ();

    #[method(name = "Transfer", args = 2)]
    pub fn transfer(self, force: crate::app::force::Force_Type, is_last: bool) -> ();

    #[method(name = "TransferForSortie", args = 2)]
    pub fn transfer_for_sortie(self, force: crate::app::force::Force_Type, is_last: bool) -> ();

    #[method(name = "TransferForRewind", args = 2)]
    pub fn transfer_for_rewind(
        self,
        force: crate::app::force::Force_Type,
        prev_unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "TransferForRewindLatest", args = 1)]
    pub fn transfer_for_rewind_latest(self, force: crate::app::force::Force_Type) -> ();

    #[method(name = "TransferImpl", args = 4)]
    pub fn transfer_impl(
        self,
        force: crate::app::force::Force_Type,
        is_last: bool,
        prev_unit: crate::app::unit::Unit,
        is_delete_actor: bool,
    ) -> ();

    #[method(name = "UpdateState", args = 0)]
    pub fn update_state(self) -> ();

    #[method(name = "UpdateStateWithAutoEquip", args = 0)]
    pub fn update_state_with_auto_equip(self) -> ();

    #[method(name = "UpdateStateWithEquipped", args = 1)]
    pub fn update_state_with_equipped(self, equipped: crate::app::unititem::UnitItem) -> ();

    #[method(name = "TryUpdateStateWithEquipped", args = 1)]
    pub fn try_update_state_with_equipped(self, equipped: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "AddSkill", args = 1)]
    pub fn add_skill(self, array: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "AddSkill", args = 2)]
    pub fn add_skill_2(
        self,
        array: crate::app::skillarray::SkillArray,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> ();

    #[method(name = "AddSkill", args = 3)]
    pub fn add_skill_3(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> ();

    #[method(name = "AddSkillWithoutUpdate", args = 3)]
    pub fn add_skill_without_update(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "IsEngaging", args = 1)]
    pub fn is_engaging(self, god_unit: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "UpdateStateImpl", args = 2)]
    pub fn update_state_impl(
        self,
        is_auto_equip: bool,
        equipped: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "CommitEnhance", args = 1)]
    pub fn commit_enhance(self, equipped: crate::app::unititem::UnitItem) -> ();

    #[method(name = "UpdatePlace", args = 0)]
    pub fn update_place(self) -> ();

    #[method(name = "CreateActor", args = 0)]
    pub fn create_actor(self) -> ();

    #[method(name = "TryCreateActor", args = 0)]
    pub fn try_create_actor(self) -> bool;

    #[method(name = "ReloadActor", args = 0)]
    pub fn reload_actor(self) -> ();

    #[method(name = "DeleteActor", args = 0)]
    pub fn delete_actor(self) -> ();

    #[method(name = "UpdateActor", args = 0)]
    pub fn update_actor(self) -> ();

    #[method(name = "TickActor", args = 0)]
    pub fn tick_actor(self) -> ();

    #[method(name = "CheckStatus", args = 1)]
    pub fn check_status(self, status: crate::app::unit::Unit_Status) -> bool;

    #[method(name = "NotStatus", args = 1)]
    pub fn not_status(self, status: crate::app::unit::Unit_Status) -> bool;

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(self, status: crate::app::unit::Unit_Status) -> ();

    #[method(name = "ClearStatus", args = 1)]
    pub fn clear_status(self, status: crate::app::unit::Unit_Status) -> ();

    #[method(name = "ChangeStatus", args = 1)]
    pub fn change_status(self, status: crate::app::unit::Unit_Status) -> ();

    #[method(name = "GetStatus", args = 0)]
    pub fn get_status(self) -> crate::app::unit::Unit_StatusField;

    #[method(name = "SetDead", args = 0)]
    pub fn set_dead(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTalkName", args = 0)]
    pub fn get_talk_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameImpl", args = 1)]
    pub fn get_name_impl(self, is_morph: bool) -> ::unity2::Il2CppString;

    #[method(name = "GetNameForRelayData", args = 5)]
    pub fn get_name_for_relay_data(
        edit: crate::app::unitedit::UnitEdit,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        is_morph: bool,
        relay_player_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetNameImpl", args = 7)]
    pub fn get_name_impl_2(
        edit: crate::app::unitedit::UnitEdit,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        is_morph: bool,
        is_guest: bool,
        relay_player_index: i32,
        force_type: crate::app::force::Force_Type,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsDefaultNameForNet", args = 3)]
    pub fn is_default_name_for_net(
        is_guest: bool,
        relay_player_index: i32,
        force_type: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "GetAsciiName", args = 0)]
    pub fn get_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetJobName", args = 0)]
    pub fn get_job_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, pos: crate::unity_engine::vector3::Vector3, update: bool) -> ();

    #[method(name = "SetPosition", args = 3)]
    pub fn set_position_2(self, x: i32, z: i32, update: bool) -> ();

    #[method(name = "SetRotation", args = 1)]
    pub fn set_rotation(self, angle: f32) -> ();

    #[method(name = "SetForceForStandAlone", args = 1)]
    pub fn set_force_for_stand_alone(self, force: crate::app::force::Force) -> ();

    #[method(name = "IsHero", args = 0)]
    pub fn is_hero(self) -> bool;

    #[method(name = "IsDead", args = 0)]
    pub fn is_dead(self) -> bool;

    #[method(name = "GetGender", args = 0)]
    pub fn get_gender(self) -> crate::app::gender::Gender;

    #[method(name = "GetDressGender", args = 0)]
    pub fn get_dress_gender(self) -> crate::app::gender::Gender;

    #[method(name = "GetGender", args = 2)]
    pub fn get_gender_2(
        edit: crate::app::unitedit::UnitEdit,
        person: crate::app::persondata::PersonData,
    ) -> crate::app::gender::Gender;

    #[method(name = "GetDressGender", args = 2)]
    pub fn get_dress_gender_2(
        edit: crate::app::unitedit::UnitEdit,
        person: crate::app::persondata::PersonData,
    ) -> crate::app::gender::Gender;

    #[method(name = "IsFemale", args = 0)]
    pub fn is_female(self) -> bool;

    #[method(name = "IsLeader", args = 0)]
    pub fn is_leader(self) -> bool;

    #[method(name = "IsMorph", args = 0)]
    pub fn is_morph(self) -> bool;

    #[method(name = "HasFangCurse", args = 0)]
    pub fn has_fang_curse(self) -> bool;

    #[method(name = "IsEnchantment", args = 0)]
    pub fn is_enchantment(self) -> bool;

    #[method(name = "IsMoveNotAllow", args = 0)]
    pub fn is_move_not_allow(self) -> bool;

    #[method(name = "IsUnique", args = 0)]
    pub fn is_unique(self) -> bool;

    #[method(name = "IsShow", args = 0)]
    pub fn is_show(self) -> bool;

    #[method(name = "CanSortie", args = 0)]
    pub fn can_sortie(self) -> bool;

    #[method(name = "IsPlayArea", args = 0)]
    pub fn is_play_area(self) -> bool;

    #[method(name = "IsPlayArea", args = 2)]
    pub fn is_play_area_2(self, x: i32, z: i32) -> bool;

    #[method(name = "IsAllied", args = 1)]
    pub fn is_allied(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsOperate", args = 0)]
    pub fn is_operate(self) -> bool;

    #[method(name = "IsEfficacy", args = 1)]
    pub fn is_efficacy(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "GetAttrs", args = 0)]
    pub fn get_attrs(self) -> crate::app::skilldata::SkillData_Attrs;

    #[method(name = "CanTransporter", args = 0)]
    pub fn can_transporter(self) -> bool;

    #[method(name = "CanBreakable", args = 1)]
    pub fn can_breakable(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "CanSkyBattle", args = 0)]
    pub fn can_sky_battle(self) -> bool;

    #[method(name = "GetForceAbsentType", args = 0)]
    pub fn get_force_absent_type(self) -> crate::app::force::Force_Type;

    #[method(name = "GetForceDeadType", args = 0)]
    pub fn get_force_dead_type(self) -> crate::app::force::Force_Type;

    #[method(name = "IsSameForceType", args = 1)]
    pub fn is_same_force_type(self, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "SetEngageImpl", args = 2)]
    pub fn set_engage_impl(self, enable: bool, link: bool) -> ();

    #[method(name = "SetEngageLinkSimulation", args = 2)]
    pub fn set_engage_link_simulation(self, enable: bool, link: crate::app::unit::Unit) -> ();

    #[method(name = "SetEngageSimulation", args = 2)]
    pub fn set_engage_simulation(self, enable: bool, link: crate::app::unit::Unit) -> ();

    #[method(name = "SetGodLinkImpl", args = 1)]
    pub fn set_god_link_impl(self, god_link: crate::app::godunit::GodUnit) -> ();

    #[method(name = "PlayEngage", args = 1)]
    pub fn play_engage(self, enable: bool) -> ();

    #[method(name = "IsEngaging", args = 0)]
    pub fn is_engaging_2(self) -> bool;

    #[method(name = "IsHeroEngaging", args = 0)]
    pub fn is_hero_engaging(self) -> bool;

    #[method(name = "CanEngageHeal", args = 0)]
    pub fn can_engage_heal(self) -> bool;

    #[method(name = "SetEngageLink", args = 4)]
    pub fn set_engage_link(
        self,
        enable: bool,
        god_link: crate::app::godunit::GodUnit,
        parent: crate::app::unit::Unit,
        child: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetEngage", args = 2)]
    pub fn set_engage(self, enable: bool, link: crate::app::unit::Unit) -> ();

    #[method(name = "TryRelocation", args = 0)]
    pub fn try_relocation(self) -> bool;

    #[method(name = "IsActiveRoute", args = 0)]
    pub fn is_active_route(self) -> bool;

    #[method(name = "get_FirstX", args = 0)]
    pub fn get_first_x(self) -> i32;

    #[method(name = "get_FirstZ", args = 0)]
    pub fn get_first_z(self) -> i32;

    #[method(name = "ForceSetEngageLinkForRewind", args = 1)]
    pub fn force_set_engage_link_for_rewind(self, link_unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetEngageAttack", args = 1)]
    pub fn set_engage_attack(self, enable: bool) -> ();

    #[method(name = "CanEngageImpl", args = 1)]
    pub fn can_engage_impl(self, god_unit: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "CanEngageCancel", args = 1)]
    pub fn can_engage_cancel(self, pickup: bool) -> bool;

    #[method(name = "CanEngageStart", args = 0)]
    pub fn can_engage_start(self) -> bool;

    #[method(name = "CanEngageLink", args = 0)]
    pub fn can_engage_link(self) -> bool;

    #[method(name = "GetLinkableGodUnit", args = 0)]
    pub fn get_linkable_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "GetEngageLinkUnit", args = 0)]
    pub fn get_engage_link_unit(self) -> crate::app::unit::Unit;

    #[method(name = "PreStartTargetSelect", args = 0)]
    pub fn pre_start_target_select(self) -> ();

    #[method(name = "PostStartTargetSelect", args = 0)]
    pub fn post_start_target_select(self) -> ();

    #[method(name = "TryAddAchieveEngage", args = 0)]
    pub fn try_add_achieve_engage(self) -> ();

    #[method(name = "DecideTargetSelect", args = 1)]
    pub fn decide_target_select(self, is_redoer: bool) -> ();

    #[method(name = "CancelTargetSelect", args = 0)]
    pub fn cancel_target_select(self) -> ();

    #[method(name = "CanChangeEngage", args = 1)]
    pub fn can_change_engage(self, target_unit: crate::app::unit::Unit) -> bool;

    #[method(name = "TryChangeEngage", args = 2)]
    pub fn try_change_engage(self, target_unit: crate::app::unit::Unit, item_index: i32) -> bool;

    #[method(name = "ChangeEngage", args = 2)]
    pub fn change_engage(self, target_unit: crate::app::unit::Unit, item_index: i32) -> ();

    #[method(name = "CanEngageAttack", args = 0)]
    pub fn can_engage_attack(self) -> bool;

    #[method(name = "CanEngageTarget", args = 1)]
    pub fn can_engage_target(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "EngageTargetExists", args = 2)]
    pub fn engage_target_exists(
        self,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "CanEnemyEngageAttack", args = 0)]
    pub fn can_enemy_engage_attack(self) -> bool;

    #[method(name = "CanExecuteEngageAttack", args = 0)]
    pub fn can_execute_engage_attack(self) -> bool;

    #[method(name = "CanAct", args = 2)]
    pub fn can_act(self, is_fixed: bool, is_charged: bool) -> bool;

    #[method(name = "CanActWithoutEngageCharge", args = 0)]
    pub fn can_act_without_engage_charge(self) -> bool;

    #[method(name = "CanBeTarget", args = 0)]
    pub fn can_be_target(self) -> bool;

    #[method(name = "CanExternalMove", args = 0)]
    pub fn can_external_move(self) -> bool;

    #[method(name = "IsSight", args = 2)]
    pub fn is_sight(self, x: i32, z: i32) -> bool;

    #[method(name = "CanWarp", args = 3)]
    pub fn can_warp(self, rod_unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "CanSkill", args = 1)]
    pub fn can_skill(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "CanChangeGod", args = 0)]
    pub fn can_change_god(self) -> bool;

    #[method(name = "ChangeGod", args = 0)]
    pub fn change_god(self) -> bool;

    #[method(name = "GetCells", args = 1)]
    pub fn get_cells(self, cells: ::unity2::Array<crate::app::mappos::MapPos>) -> i32;

    #[method(name = "GetCells", args = 3)]
    pub fn get_cells_2(
        self,
        cells: ::unity2::Array<crate::app::mappos::MapPos>,
        base_x: i32,
        base_z: i32,
    ) -> i32;

    #[method(name = "SetBaseCapability", args = 2)]
    pub fn set_base_capability(self, index: i32, value: i32) -> ();

    #[method(name = "AddBaseCapability", args = 2)]
    pub fn add_base_capability(self, index: i32, value: i32) -> ();

    #[method(name = "SetCapabilityJust", args = 2)]
    pub fn set_capability_just(self, index: i32, value: i32) -> ();

    #[method(name = "GetGodEnhanceRating", args = 1)]
    pub fn get_god_enhance_rating(self, god: crate::app::godunit::GodUnit) -> i32;

    #[method(name = "ChangeHp", args = 1)]
    pub fn change_hp(self, hp: i32) -> ();

    #[method(name = "PlaySetHp", args = 1)]
    pub fn play_set_hp(self, hp: i32) -> ();

    #[method(name = "PlaySetHeal", args = 1)]
    pub fn play_set_heal(self, heal: i32) -> ();

    #[method(name = "PlaySetDamage", args = 3)]
    pub fn play_set_damage(self, damage: i32, can_die: bool, is_multi: bool) -> ();

    #[method(name = "IsChangingHp", args = 0)]
    pub fn is_changing_hp(self) -> bool;

    #[method(name = "GetMaxHp", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "GetStr", args = 0)]
    pub fn get_str(self) -> i32;

    #[method(name = "GetTech", args = 0)]
    pub fn get_tech(self) -> i32;

    #[method(name = "GetQuick", args = 0)]
    pub fn get_quick(self) -> i32;

    #[method(name = "GetLuck", args = 0)]
    pub fn get_luck(self) -> i32;

    #[method(name = "GetDef", args = 0)]
    pub fn get_def(self) -> i32;

    #[method(name = "GetMagic", args = 0)]
    pub fn get_magic(self) -> i32;

    #[method(name = "GetMdef", args = 0)]
    pub fn get_mdef(self) -> i32;

    #[method(name = "GetPhys", args = 0)]
    pub fn get_phys(self) -> i32;

    #[method(name = "GetSight", args = 0)]
    pub fn get_sight(self) -> i32;

    #[method(name = "GetMovePower", args = 0)]
    pub fn get_move_power(self) -> i32;

    #[method(name = "GetMovePowerWithoutRemoving", args = 0)]
    pub fn get_move_power_without_removing(self) -> i32;

    #[method(name = "FindSkill", args = 1)]
    pub fn find_skill(
        self,
        flags: crate::app::skilldata::SkillData_Flags,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "FindSkill", args = 1)]
    pub fn find_skill_2(
        self,
        states: crate::app::skilldata::SkillData_States,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "GetRemovableSkill", args = 0)]
    pub fn get_removable_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "GetRemovablePower", args = 0)]
    pub fn get_removable_power(self) -> i32;

    #[method(name = "GetCapability", args = 2)]
    pub fn get_capability(self, index: i32, calc_enhance: bool) -> i32;

    #[method(name = "GetCapability", args = 2)]
    pub fn get_capability_2(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        calc_enhance: bool,
    ) -> i32;

    #[method(name = "GetCapabilityGrow", args = 2)]
    pub fn get_capability_grow(self, index: i32, is_auto_grow: bool) -> i32;

    #[method(name = "GetCapabilityGrow", args = 2)]
    pub fn get_capability_grow_2(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        is_auto_grow: bool,
    ) -> i32;

    #[method(name = "GetCapabilityLimit", args = 1)]
    pub fn get_capability_limit(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> i32;

    #[method(name = "GetCapabilityLimit", args = 1)]
    pub fn get_capability_limit_2(self, index: i32) -> i32;

    #[method(name = "CanCapabilityGrow", args = 1)]
    pub fn can_capability_grow(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> bool;

    #[method(name = "CanCapabilityGrow", args = 1)]
    pub fn can_capability_grow_2(self, index: i32) -> bool;

    #[method(name = "GetStrengthCapabilityIndex", args = 1)]
    pub fn get_strength_capability_index(
        self,
        rand: crate::app::random_2::Random_2,
    ) -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = "GetNoEnhanceMaxHp", args = 0)]
    pub fn get_no_enhance_max_hp(self) -> i32;

    #[method(name = "GetNoEnhanceStr", args = 0)]
    pub fn get_no_enhance_str(self) -> i32;

    #[method(name = "GetNoEnhanceTech", args = 0)]
    pub fn get_no_enhance_tech(self) -> i32;

    #[method(name = "GetNoEnhanceQuick", args = 0)]
    pub fn get_no_enhance_quick(self) -> i32;

    #[method(name = "GetNoEnhanceLuck", args = 0)]
    pub fn get_no_enhance_luck(self) -> i32;

    #[method(name = "GetNoEnhanceDef", args = 0)]
    pub fn get_no_enhance_def(self) -> i32;

    #[method(name = "GetNoEnhanceMagic", args = 0)]
    pub fn get_no_enhance_magic(self) -> i32;

    #[method(name = "GetNoEnhanceMdef", args = 0)]
    pub fn get_no_enhance_mdef(self) -> i32;

    #[method(name = "GetNoEnhancePhys", args = 0)]
    pub fn get_no_enhance_phys(self) -> i32;

    #[method(name = "GetNoEnhanceSight", args = 0)]
    pub fn get_no_enhance_sight(self) -> i32;

    #[method(name = "GetNoEnhanceMovePower", args = 0)]
    pub fn get_no_enhance_move_power(self) -> i32;

    #[method(name = "GetNoEnhanceCapability", args = 1)]
    pub fn get_no_enhance_capability(self, index: i32) -> i32;

    #[method(name = "GetNoEnhanceCapability", args = 1)]
    pub fn get_no_enhance_capability_2(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> i32;

    #[method(name = "GetEnhancedLevel", args = 0)]
    pub fn get_enhanced_level(self) -> i32;

    #[method(name = "AddHp", args = 1)]
    pub fn add_hp(self, v: i32) -> ();

    #[method(name = "IsDontAttack", args = 1)]
    pub fn is_dont_attack(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetDontAttack", args = 1)]
    pub fn set_dont_attack(self, person: crate::app::persondata::PersonData) -> ();

    #[method(name = "SetDontAttackForceMask", args = 1)]
    pub fn set_dont_attack_force_mask(self, force_mask: u32) -> ();

    #[method(name = "GetEntrustForAI", args = 0)]
    pub fn get_entrust_for_ai(self) -> crate::app::unitentrust::UnitEntrust_Type;

    #[method(name = "CanTalk", args = 0)]
    pub fn can_talk(self) -> bool;

    #[method(name = "CanDance", args = 1)]
    pub fn can_dance(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "CanContract", args = 1)]
    pub fn can_contract(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAgain", args = 0)]
    pub fn can_again(self) -> bool;

    #[method(name = "CanChain", args = 1)]
    pub fn can_chain(self, parent: crate::app::unit::Unit) -> bool;

    #[method(name = "CanChainAttack", args = 0)]
    pub fn can_chain_attack(self) -> bool;

    #[method(name = "IsTerrainInvalid", args = 1)]
    pub fn is_terrain_invalid(self, terrain: crate::app::terraindata_2::TerrainData_2) -> bool;

    #[method(name = "CanChainGuard", args = 0)]
    pub fn can_chain_guard(self) -> bool;

    #[method(name = "GetGuardType", args = 0)]
    pub fn get_guard_type(self) -> crate::app::unit::Unit_GuardType;

    #[method(name = "CanCcogitation", args = 0)]
    pub fn can_ccogitation(self) -> bool;

    #[method(name = "CanTrade", args = 0)]
    pub fn can_trade(self) -> bool;

    #[method(name = "CanGainReliance", args = 0)]
    pub fn can_gain_reliance(self) -> bool;

    #[method(name = "HasSkill", args = 1)]
    pub fn has_skill(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "HasSkill", args = 1)]
    pub fn has_skill_2(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasSkill", args = 1)]
    pub fn has_skill_3(self, flags: crate::app::skilldata::SkillData_Flags) -> bool;

    #[method(name = "HasSkill", args = 1)]
    pub fn has_skill_4(self, states: crate::app::skilldata::SkillData_States) -> bool;

    #[method(name = "HasDanceSkill", args = 0)]
    pub fn has_dance_skill(self) -> bool;

    #[method(name = "HasContractSkill", args = 0)]
    pub fn has_contract_skill(self) -> bool;

    #[method(name = "IsStandingDie", args = 0)]
    pub fn is_standing_die(self) -> bool;

    #[method(name = "IsImmortal", args = 0)]
    pub fn is_immortal(self) -> bool;

    #[method(name = "IsLastBoss", args = 0)]
    pub fn is_last_boss(self) -> bool;

    #[method(name = "GetEquipSkill", args = 1)]
    pub fn get_equip_skill(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "HasEquipSkill", args = 1)]
    pub fn has_equip_skill(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasEquipSkill", args = 1)]
    pub fn has_equip_skill_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "AddEquipSkill", args = 1)]
    pub fn add_equip_skill(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "AddEquipSkill", args = 1)]
    pub fn add_equip_skill_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "RemoveEquipSkill", args = 1)]
    pub fn remove_equip_skill(self, sid: ::unity2::Il2CppString) -> ();

    #[method(name = "RemoveEquipSkill", args = 1)]
    pub fn remove_equip_skill_2(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "ReplaceEquipSkill", args = 2)]
    pub fn replace_equip_skill(self, index: i32, sid: ::unity2::Il2CppString) -> ();

    #[method(name = "ReplaceEquipSkill", args = 2)]
    pub fn replace_equip_skill_2(self, index: i32, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "MoveEquipSkill", args = 2)]
    pub fn move_equip_skill(self, old_index: i32, new_index: i32) -> ();

    #[method(name = "HasPrivateSkill", args = 1)]
    pub fn has_private_skill(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasPrivateSkill", args = 1)]
    pub fn has_private_skill_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "AddPrivateSkill", args = 1)]
    pub fn add_private_skill(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "AddPrivateSkill", args = 1)]
    pub fn add_private_skill_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "AddPrivateSkill", args = 1)]
    pub fn add_private_skill_3(self, skills: crate::app::skillarray::SkillArray) -> bool;

    #[method(name = "RemovePrivateSkill", args = 1)]
    pub fn remove_private_skill(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "RemovePrivateSkill", args = 1)]
    pub fn remove_private_skill_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "RemovePrivateSkill", args = 1)]
    pub fn remove_private_skill_3(self, skills: crate::app::skillarray::SkillArray) -> bool;

    #[method(name = "GetRecord", args = 1)]
    pub fn get_record(self, kind: crate::app::unitrecord::UnitRecord_Kinds) -> i32;

    #[method(name = "SetRecord", args = 2)]
    pub fn set_record(self, kind: crate::app::unitrecord::UnitRecord_Kinds, value: i32) -> ();

    #[method(name = "AddRecord", args = 2)]
    pub fn add_record(self, kind: crate::app::unitrecord::UnitRecord_Kinds, value: i32) -> ();

    #[method(name = "IncRecord", args = 1)]
    pub fn inc_record(self, kind: crate::app::unitrecord::UnitRecord_Kinds) -> ();

    #[method(name = "DecRecord", args = 1)]
    pub fn dec_record(self, kind: crate::app::unitrecord::UnitRecord_Kinds) -> ();

    #[method(name = "AddRecordWithHistory", args = 2)]
    pub fn add_record_with_history(
        self,
        kind: crate::app::unitrecord::UnitRecord_Kinds,
        value: i32,
    ) -> ();

    #[method(name = "GetFromEquipSkillPool", args = 1)]
    pub fn get_from_equip_skill_pool(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "AddToEquipSkillPool", args = 1)]
    pub fn add_to_equip_skill_pool(self, sid: ::unity2::Il2CppString) -> ();

    #[method(name = "AddToEquipSkillPool", args = 1)]
    pub fn add_to_equip_skill_pool_2(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "RemoveFromEquipSkillPool", args = 1)]
    pub fn remove_from_equip_skill_pool(self, sid: ::unity2::Il2CppString) -> ();

    #[method(name = "RemoveFromEquipSkillPool", args = 1)]
    pub fn remove_from_equip_skill_pool_2(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "IsExistInEquipSkillPool", args = 1)]
    pub fn is_exist_in_equip_skill_pool(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsExistInEquipSkillPool", args = 1)]
    pub fn is_exist_in_equip_skill_pool_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "IsInheritanceEnable", args = 1)]
    pub fn is_inheritance_enable(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "IsPoison", args = 0)]
    pub fn is_poison(self) -> bool;

    #[method(name = "IsStun", args = 0)]
    pub fn is_stun(self) -> bool;

    #[method(name = "IsSleep", args = 0)]
    pub fn is_sleep(self) -> bool;

    #[method(name = "IsSilence", args = 0)]
    pub fn is_silence(self) -> bool;

    #[method(name = "IsCharm", args = 0)]
    pub fn is_charm(self) -> bool;

    #[method(name = "IsConfusion", args = 0)]
    pub fn is_confusion(self) -> bool;

    #[method(name = "IsCharmOrConfusion", args = 0)]
    pub fn is_charm_or_confusion(self) -> bool;

    #[method(name = "IsFreeze", args = 0)]
    pub fn is_freeze(self) -> bool;

    #[method(name = "IsWeakness", args = 0)]
    pub fn is_weakness(self) -> bool;

    #[method(name = "IsNotEnhance", args = 0)]
    pub fn is_not_enhance(self) -> bool;

    #[method(name = "IsDisorder", args = 0)]
    pub fn is_disorder(self) -> bool;

    #[method(name = "ClearDisorder", args = 1)]
    pub fn clear_disorder(self, state: crate::app::skilldata::SkillData_States) -> bool;

    #[method(name = "IsVision", args = 0)]
    pub fn is_vision(self) -> bool;

    #[method(name = "IsVision", args = 1)]
    pub fn is_vision_2(self, owner: crate::app::unit::Unit) -> bool;

    #[method(name = "CanGainItem", args = 0)]
    pub fn can_gain_item(self) -> bool;

    #[method(name = "GetVisionOwner", args = 0)]
    pub fn get_vision_owner(self) -> crate::app::unit::Unit;

    #[method(name = "IsSummon", args = 0)]
    pub fn is_summon(self) -> bool;

    #[method(name = "IsSummon", args = 1)]
    pub fn is_summon_2(self, owner: crate::app::unit::Unit) -> bool;

    #[method(name = "IsSummonGod", args = 0)]
    pub fn is_summon_god(self) -> bool;

    #[method(name = "GetSummonOwner", args = 0)]
    pub fn get_summon_owner(self) -> crate::app::unit::Unit;

    #[method(name = "IsLockon", args = 0)]
    pub fn is_lockon(self) -> bool;

    #[method(name = "TryGetLockTarget", args = 2)]
    pub fn try_get_lock_target(self, x: i32, z: i32) -> bool;

    #[method(name = "SetLockTarget", args = 2)]
    pub fn set_lock_target(self, x: i32, z: i32) -> ();

    #[method(name = "ResetLockTarget", args = 0)]
    pub fn reset_lock_target(self) -> ();

    #[method(name = "IsOnMap", args = 0)]
    pub fn is_on_map(self) -> bool;

    #[method(name = "IsDecoy", args = 0)]
    pub fn is_decoy(self) -> bool;

    #[method(name = "AddExp", args = 1)]
    pub fn add_exp(self, exp: i32) -> ();

    #[method(name = "AddSkillPoint", args = 1)]
    pub fn add_skill_point(self, skill_point: i32) -> ();

    #[method(name = "CanGrow", args = 0)]
    pub fn can_grow(self) -> bool;

    #[method(name = "NormalizeExp", args = 1)]
    pub fn normalize_exp(self, exp: i32) -> i32;

    #[method(name = "ExpToSkillPoint", args = 1)]
    pub fn exp_to_skill_point(self, exp: i32) -> i32;

    #[method(name = "NomralizeSkillPoint", args = 1)]
    pub fn nomralize_skill_point(self, skill_point: i32) -> i32;

    #[method(name = "LevelUp", args = 1)]
    pub fn level_up(self, abort: i32) -> ();

    #[method(name = "DbgLevelDown", args = 0)]
    pub fn dbg_level_down(self) -> ();

    #[method(name = "LearnJobSkill", args = 0)]
    pub fn learn_job_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "LearnJobSkillForChart", args = 1)]
    pub fn learn_job_skill_for_chart(self, level: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "LearnJobSkill", args = 2)]
    pub fn learn_job_skill_2(
        self,
        level: i32,
        job: crate::app::jobdata::JobData,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "LearnJobSkill", args = 1)]
    pub fn learn_job_skill_3(
        self,
        job: crate::app::jobdata::JobData,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "CCCheckAptitude", args = 1)]
    pub fn cc_check_aptitude(self, job: crate::app::jobdata::JobData) -> bool;

    #[method(name = "ClassChange", args = 2)]
    pub fn class_change(
        self,
        job: crate::app::jobdata::JobData,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "ClassChangeForChart", args = 3)]
    pub fn class_change_for_chart(
        self,
        job: crate::app::jobdata::JobData,
        item: crate::app::itemdata::ItemData,
        level: i32,
    ) -> ();

    #[method(name = "UpdateDifficulty", args = 0)]
    pub fn update_difficulty(self) -> ();

    #[method(name = "ResetWeaponMask", args = 0)]
    pub fn reset_weapon_mask(self) -> ();

    #[method(name = "SetSelectedWeapon", args = 1)]
    pub fn set_selected_weapon(
        self,
        selected_weapon_mask: crate::app::weaponmask::WeaponMask,
    ) -> ();

    #[method(name = "SetSelectedWeaponFromOriginalAptitude", args = 1)]
    pub fn set_selected_weapon_from_original_aptitude(
        self,
        aptitude: crate::app::weaponmask::WeaponMask,
    ) -> ();

    #[method(name = "SetWeaponMaskFromParson", args = 0)]
    pub fn set_weapon_mask_from_parson(self) -> ();

    #[method(name = "SetOptimalWeaponForClassChange", args = 2)]
    pub fn set_optimal_weapon_for_class_change(
        self,
        weapon_mask: crate::app::weaponmask::WeaponMask,
        is_bullet: bool,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "SetSelectedWeaponFromItems", args = 1)]
    pub fn set_selected_weapon_from_items(
        self,
        items: crate::app::unit::Unit_ItemsForSelectedWeapon,
    ) -> ();

    #[method(name = "SetSelectedWeaponForChart", args = 1)]
    pub fn set_selected_weapon_for_chart(
        self,
        chart_items: ::unity2::Array<crate::app::chartdata::ChartData_Item>,
    ) -> ();

    #[method(name = "SetSelectedWeaponFromDispos", args = 1)]
    pub fn set_selected_weapon_from_dispos(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "InheritAptitude", args = 1)]
    pub fn inherit_aptitude(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "InheritAptitude", args = 1)]
    pub fn inherit_aptitude_2(self, god_bond: crate::app::godbond::GodBond) -> ();

    #[method(name = "AddAptitudeForChart", args = 0)]
    pub fn add_aptitude_for_chart(self) -> ();

    #[method(name = "SetAptitudeFromDispos", args = 1)]
    pub fn set_aptitude_from_dispos(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "AddAptitudeFromWeaponMask", args = 0)]
    pub fn add_aptitude_from_weapon_mask(self) -> ();

    #[method(name = "UpdateWeaponMask", args = 0)]
    pub fn update_weapon_mask(self) -> ();

    #[method(name = "AIActivate", args = 1)]
    pub fn ai_activate(self, attacked: bool) -> ();

    #[method(name = "AIActivate_CauseAttacked", args = 1)]
    pub fn ai_activate_cause_attacked(self, longrange: bool) -> ();

    #[method(name = "AISetEngageAttack", args = 0)]
    pub fn ai_set_engage_attack(self) -> ();

    #[method(name = "AIClearEngageAttack", args = 0)]
    pub fn ai_clear_engage_attack(self) -> ();

    #[method(name = "HasItem", args = 1)]
    pub fn has_item(self, is_exclude_engage: bool) -> bool;

    #[method(name = "HasRod", args = 0)]
    pub fn has_rod(self) -> bool;

    #[method(name = "HasHealRod", args = 0)]
    pub fn has_heal_rod(self) -> bool;

    #[method(name = "HasHealRodForOneself", args = 1)]
    pub fn has_heal_rod_for_oneself(
        self,
        rod_type: crate::app::itemdata::ItemData_RodTypes,
    ) -> bool;

    #[method(name = "HasSupportRod", args = 0)]
    pub fn has_support_rod(self) -> bool;

    #[method(name = "HasSupportRodForOneself", args = 1)]
    pub fn has_support_rod_for_oneself(
        self,
        rod_type: crate::app::itemdata::ItemData_RodTypes,
    ) -> bool;

    #[method(name = "HasInterferenceRod", args = 0)]
    pub fn has_interference_rod(self) -> bool;

    #[method(name = "HasCriticaleWeapon", args = 0)]
    pub fn has_criticale_weapon(self) -> bool;

    #[method(name = "HasEfficacyWeapon", args = 1)]
    pub fn has_efficacy_weapon(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "HasDropItem", args = 0)]
    pub fn has_drop_item(self) -> bool;

    #[method(name = "HasMaxItem", args = 0)]
    pub fn has_max_item(self) -> bool;

    #[method(name = "GetItem", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::unititem::UnitItem;

    #[method(name = "GetItemIndex", args = 1)]
    pub fn get_item_index(self, iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetItemIndex", args = 1)]
    pub fn get_item_index_2(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "ForEachUnitItem", args = 1)]
    pub fn for_each_unit_item(self, func: crate::app::unit::Unit_FuncUnitItem) -> ();

    #[method(name = "ForEachCanEquipItem", args = 1)]
    pub fn for_each_can_equip_item(self, func: crate::app::unit::Unit_FuncUnitItem) -> ();

    #[method(name = "GetActualItem", args = 1)]
    pub fn get_actual_item(
        self,
        unit_item: crate::app::unititem::UnitItem,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "IsItemEquipped", args = 0)]
    pub fn is_item_equipped(self) -> bool;

    #[method(name = "GetItemEquipped", args = 0)]
    pub fn get_item_equipped(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetItemIndexEquipped", args = 0)]
    pub fn get_item_index_equipped(self) -> i32;

    #[method(name = "GetItemHold", args = 0)]
    pub fn get_item_hold(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetItemIndexHold", args = 0)]
    pub fn get_item_index_hold(self) -> i32;

    #[method(name = "ItemAdd", args = 1)]
    pub fn item_add(self, iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "ItemAdd", args = 1)]
    pub fn item_add_2(self, iids: ::unity2::Array<::unity2::Il2CppString>) -> bool;

    #[method(name = "ItemAdd", args = 1)]
    pub fn item_add_3(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "ItemAdd", args = 1)]
    pub fn item_add_4(self, unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "ItemAddOnDlcEvil", args = 3)]
    pub fn item_add_on_dlc_evil(
        self,
        iids: ::unity2::Array<::unity2::Il2CppString>,
        chapter: crate::app::chapterdata::ChapterData,
        level: i32,
    ) -> ();

    #[method(name = "EquipableItemAdd", args = 1)]
    pub fn equipable_item_add(self, iids: ::unity2::Array<::unity2::Il2CppString>) -> bool;

    #[method(name = "ItemMove", args = 2)]
    pub fn item_move(self, from: i32, to: i32) -> ();

    #[method(name = "ItemCloseUp", args = 0)]
    pub fn item_close_up(self) -> ();

    #[method(name = "ItemEquip", args = 0)]
    pub fn item_equip(self) -> bool;

    #[method(name = "ItemEquip", args = 2)]
    pub fn item_equip_2(self, index: i32, reoder: bool) -> bool;

    #[method(name = "ItemReorder", args = 0)]
    pub fn item_reorder(self) -> ();

    #[method(name = "ItemEquip", args = 1)]
    pub fn item_equip_3(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "ItemEquip", args = 1)]
    pub fn item_equip_4(self, item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "ItemTakeOff", args = 1)]
    pub fn item_take_off(self, index: i32) -> ();

    #[method(name = "ItemTakeOff", args = 1)]
    pub fn item_take_off_2(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "ItemPutOff", args = 2)]
    pub fn item_put_off(self, index: i32, closeup: bool) -> ();

    #[method(name = "ItemPutOff", args = 2)]
    pub fn item_put_off_2(self, unit_item: crate::app::unititem::UnitItem, closeup: bool) -> ();

    #[method(name = "ItemPutOffAll", args = 0)]
    pub fn item_put_off_all(self) -> ();

    #[method(name = "HasItem", args = 1)]
    pub fn has_item_2(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "GetItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "GetItemSelected", args = 0)]
    pub fn get_item_selected(self) -> crate::app::unititem::UnitItem;

    #[method(name = "SetItemSelected", args = 1)]
    pub fn set_item_selected(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "HasEquipableItem", args = 0)]
    pub fn has_equipable_item(self) -> bool;

    #[method(name = "HasEquipableItem", args = 1)]
    pub fn has_equipable_item_2(self, range: i32) -> bool;

    #[method(name = "HasEquipableItem", args = 1)]
    pub fn has_equipable_item_3(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "CanItemEquip", args = 3)]
    pub fn can_item_equip(
        self,
        unit_item: crate::app::unititem::UnitItem,
        rod: bool,
        exp: bool,
    ) -> bool;

    #[method(name = "CanItemEquip", args = 3)]
    pub fn can_item_equip_2(
        self,
        item: crate::app::itemdata::ItemData,
        rod: bool,
        weapon_level: bool,
    ) -> bool;

    #[method(name = "CanItemEquip", args = 3)]
    pub fn can_item_equip_3(self, index: i32, rod: bool, exp: bool) -> bool;

    #[method(name = "CanItemKindEquip", args = 3)]
    pub fn can_item_kind_equip(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
        rod: bool,
        weapon_mask: bool,
    ) -> bool;

    #[method(name = "CanUseCannon", args = 2)]
    pub fn can_use_cannon(self, x: i32, z: i32) -> bool;

    #[method(name = "CanUseCannon", args = 1)]
    pub fn can_use_cannon_2(self, terrain: crate::app::terraindata_2::TerrainData_2) -> bool;

    #[method(name = "NextItemEquip", args = 1)]
    pub fn next_item_equip(self, reverse: bool) -> bool;

    #[method(name = "IsItemSealed", args = 1)]
    pub fn is_item_sealed(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsItemSealed", args = 1)]
    pub fn is_item_sealed_2(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "IsItemSealed", args = 1)]
    pub fn is_item_sealed_3(self, index: i32) -> bool;

    #[method(name = "GetItemIndexKeyDoor", args = 0)]
    pub fn get_item_index_key_door(self) -> i32;

    #[method(name = "CanUnlockDoor", args = 1)]
    pub fn can_unlock_door(self, enable_item: bool) -> bool;

    #[method(name = "CanUnlockTreasureBox", args = 1)]
    pub fn can_unlock_treasure_box(self, enable_item: bool) -> bool;

    #[method(name = "GetRangeI", args = 2)]
    pub fn get_range_i(self, index: i32, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "GetRangeI", args = 2)]
    pub fn get_range_i_2(
        self,
        item: crate::app::itemdata::ItemData,
        skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "GetRangeI", args = 2)]
    pub fn get_range_i_3(
        self,
        unit_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "GetRangeO", args = 2)]
    pub fn get_range_o(self, index: i32, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "GetRangeO", args = 2)]
    pub fn get_range_o_2(
        self,
        item: crate::app::itemdata::ItemData,
        skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "GetRangeO", args = 2)]
    pub fn get_range_o_3(
        self,
        unit_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
    ) -> i32;

    #[method(name = "GetRangeIO", args = 4)]
    pub fn get_range_io(
        self,
        index: i32,
        range_i: i32,
        range_o: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetRangeIO", args = 4)]
    pub fn get_range_io_2(
        self,
        item: crate::app::itemdata::ItemData,
        range_i: i32,
        range_o: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetRangeIO", args = 4)]
    pub fn get_range_io_3(
        self,
        unit_item: crate::app::unititem::UnitItem,
        range_i: i32,
        range_o: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetItemDistance", args = 1)]
    pub fn get_item_distance(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetRodRangeExtend", args = 1)]
    pub fn get_rod_range_extend(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetItemRange", args = 4)]
    pub fn get_item_range(
        self,
        min_range: i32,
        max_range: i32,
        item: crate::app::itemdata::ItemData,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetSkillRange", args = 3)]
    pub fn get_skill_range(
        self,
        min_range: i32,
        max_range: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetItemRange", args = 4)]
    pub fn get_item_range_2(
        self,
        min_range: i32,
        max_range: i32,
        equip_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetItemRange", args = 4)]
    pub fn get_item_range_3(
        self,
        min_range: i32,
        max_range: i32,
        item_mask: u32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetEngageRange", args = 2)]
    pub fn get_engage_range(self, min_range: i32, max_range: i32) -> bool;

    #[method(name = "GetAttackRange", args = 3)]
    pub fn get_attack_range(
        self,
        min_range: i32,
        max_range: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetRodRange", args = 3)]
    pub fn get_rod_range(
        self,
        min_range: i32,
        max_range: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetAttackRange", args = 4)]
    pub fn get_attack_range_2(
        self,
        min_range: i32,
        max_range: i32,
        equip_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetRodRange", args = 4)]
    pub fn get_rod_range_2(
        self,
        min_range: i32,
        max_range: i32,
        equip_item: crate::app::unititem::UnitItem,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetRevengeWeapon", args = 3)]
    pub fn get_revenge_weapon(
        self,
        target: crate::app::unit::Unit,
        target_item: crate::app::unititem::UnitItem,
        range: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetRodHealPower", args = 1)]
    pub fn get_rod_heal_power(self, unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "CanItemUse", args = 1)]
    pub fn can_item_use(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "CanItemUse", args = 2)]
    pub fn can_item_use_2(
        self,
        item: crate::app::itemdata::ItemData,
        target_unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "ItemUseImpl", args = 4)]
    pub fn item_use_impl(
        self,
        item: crate::app::itemdata::ItemData,
        use_type: crate::app::itemdata::ItemData_UseTypes,
        power: i32,
        give_skills: crate::app::skillarray::SkillArray,
    ) -> ();

    #[method(name = "ItemUse", args = 1)]
    pub fn item_use(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "ItemEnchant", args = 2)]
    pub fn item_enchant(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "IsItemEnhanceHaving", args = 1)]
    pub fn is_item_enhance_having(self, item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "IsDrawActiveColor", args = 1)]
    pub fn is_draw_active_color(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "get_AccessoryList", args = 0)]
    pub fn get_accessory_list(self) -> crate::app::unitaccessorylist::UnitAccessoryList;

    #[method(name = "SetRing", args = 1)]
    pub fn set_ring(self, ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "SetRingImpl", args = 1)]
    pub fn set_ring_impl(self, ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "ClearRing", args = 0)]
    pub fn clear_ring(self) -> ();

    #[method(name = "ClearRingImpl", args = 0)]
    pub fn clear_ring_impl(self) -> ();

    #[method(name = "get_Ring", args = 0)]
    pub fn get_ring(self) -> crate::app::unitring::UnitRing;

    #[method(name = "GetPower", args = 0)]
    pub fn get_power(self) -> i32;

    #[method(name = "SetGodUnit", args = 1)]
    pub fn set_god_unit(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "SetGodUnitToCopy", args = 1)]
    pub fn set_god_unit_to_copy(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "ClearGodUnit", args = 0)]
    pub fn clear_god_unit(self) -> ();

    #[method(name = "ClearGodUnitFromCopy", args = 0)]
    pub fn clear_god_unit_from_copy(self) -> ();

    #[method(name = "CanGodEquip", args = 1)]
    pub fn can_god_equip(self, god_unit: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "ResetEngageCount", args = 0)]
    pub fn reset_engage_count(self) -> ();

    #[method(name = "PlayEngageCancel", args = 0)]
    pub fn play_engage_cancel(self) -> ();

    #[method(name = "ResetEngaing", args = 0)]
    pub fn reset_engaing(self) -> ();

    #[method(name = "TryConnectGodUnit", args = 1)]
    pub fn try_connect_god_unit(
        self,
        god_unit: crate::app::godunit::GodUnit,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "TryConnectGodUnitToCopy", args = 1)]
    pub fn try_connect_god_unit_to_copy(
        self,
        god_unit: crate::app::godunit::GodUnit,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "TryDisconnectGodUnit", args = 0)]
    pub fn try_disconnect_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "TryDisconnectRing", args = 0)]
    pub fn try_disconnect_ring(self) -> crate::app::unitring::UnitRing;

    #[method(name = "TryDisconnectGodUnitFromCopy", args = 0)]
    pub fn try_disconnect_god_unit_from_copy(self) -> crate::app::godunit::GodUnit;

    #[method(name = "TryDisconnectGodLinkFromCopy", args = 0)]
    pub fn try_disconnect_god_link_from_copy(self) -> crate::app::godunit::GodUnit;

    #[method(name = "SightUp", args = 2)]
    pub fn sight_up(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "CanRevive", args = 0)]
    pub fn can_revive(self) -> bool;

    #[method(name = "IsAsphyxiation", args = 0)]
    pub fn is_asphyxiation(self) -> bool;

    #[method(name = "Revive", args = 0)]
    pub fn revive(self) -> ();

    #[method(name = "ReviveForRewindV0", args = 1)]
    pub fn revive_for_rewind_v0(self, hp_stock_count: u8) -> ();

    #[method(name = "ReviveForRewindV1", args = 3)]
    pub fn revive_for_rewind_v1(
        self,
        plain_hp_stock_count: u8,
        extra_hp_stock_count: u8,
        extra_hp_stock_count_max: u8,
    ) -> ();

    #[method(name = "SetGodState", args = 2)]
    pub fn set_god_state(self, index: i32, state: crate::app::godstate::GodState) -> ();

    #[method(name = "GetGodState", args = 1)]
    pub fn get_god_state(self, index: i32) -> crate::app::godstate::GodState;

    #[method(name = "GetGodStateCount", args = 0)]
    pub fn get_god_state_count(self) -> i32;

    #[method(name = "IsDefaultGodStates", args = 0)]
    pub fn is_default_god_states(self) -> bool;

    #[method(name = "GetCurrentGodStateIndex", args = 0)]
    pub fn get_current_god_state_index(self) -> i32;

    #[method(name = "GetCurrentGodState", args = 0)]
    pub fn get_current_god_state(self) -> crate::app::godstate::GodState;

    #[method(name = "CanSetExtraHpStock", args = 0)]
    pub fn can_set_extra_hp_stock(self) -> bool;

    #[method(name = "HasExtraHpStock", args = 0)]
    pub fn has_extra_hp_stock(self) -> bool;

    #[method(name = "SetExtraHpStock", args = 0)]
    pub fn set_extra_hp_stock(self) -> ();

    #[method(name = "ClearExtraHpStock", args = 0)]
    pub fn clear_extra_hp_stock(self) -> ();

    #[method(name = "ExtraHpStockForRewind", args = 3)]
    pub fn extra_hp_stock_for_rewind(
        self,
        is_set: bool,
        extra_hp_stock_count: u8,
        extra_hp_stock_count_max: u8,
    ) -> ();

    #[method(name = "DbgSetHpStockCount", args = 1)]
    pub fn dbg_set_hp_stock_count(self, count: i32) -> ();

    #[method(name = "UpdateGodUnitForGodState", args = 2)]
    pub fn update_god_unit_for_god_state(
        self,
        new_god_state: crate::app::godstate::GodState,
        old_god_state: crate::app::godstate::GodState,
    ) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Prev", args = 0)]
    pub fn get_prev(self) -> crate::app::unit::Unit;

    #[method(name = "set_Prev", args = 1)]
    pub fn set_prev(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Next", args = 0)]
    pub fn get_next(self) -> crate::app::unit::Unit;

    #[method(name = "set_Next", args = 1)]
    pub fn set_next(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Ai", args = 0)]
    pub fn get_ai(self) -> crate::app::unitai::UnitAI;

    #[method(name = "get_Edit", args = 0)]
    pub fn get_edit(self) -> crate::app::unitedit::UnitEdit;

    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: crate::app::persondata::PersonData) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PrefixlessPid", args = 0)]
    pub fn get_prefixless_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "set_Job", args = 1)]
    pub fn set_job(self, value: crate::app::jobdata::JobData) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PrefixlessJid", args = 0)]
    pub fn get_prefixless_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Force", args = 0)]
    pub fn get_force(self) -> crate::app::force::Force;

    #[method(name = "set_Force", args = 1)]
    pub fn set_force(self, value: crate::app::force::Force) -> ();

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_ForceMask", args = 0)]
    pub fn get_force_mask(self) -> u32;

    #[method(name = "get_BaseCapability", args = 0)]
    pub fn get_base_capability(self) -> crate::app::unitbasecapability::UnitBaseCapability;

    #[method(name = "get_GrowCapability", args = 0)]
    pub fn get_grow_capability(self) -> crate::app::capability::Capability;

    #[method(name = "get_LevelCapability", args = 0)]
    pub fn get_level_capability(self) -> crate::app::unitbasecapability::UnitBaseCapability;

    #[method(name = "get_GrowSeed", args = 0)]
    pub fn get_grow_seed(self) -> u32;

    #[method(name = "set_GrowSeed", args = 1)]
    pub fn set_grow_seed(self, value: u32) -> ();

    #[method(name = "get_DropSeed", args = 0)]
    pub fn get_drop_seed(self) -> u32;

    #[method(name = "set_DropSeed", args = 1)]
    pub fn set_drop_seed(self, value: u32) -> ();

    #[method(name = "get_Actor", args = 0)]
    pub fn get_actor(self) -> crate::app::unitactor::UnitActor;

    #[method(name = "set_Actor", args = 1)]
    pub fn set_actor(self, value: crate::app::unitactor::UnitActor) -> ();

    #[method(name = "get_Info", args = 0)]
    pub fn get_info(self) -> crate::app::mapinforoot::MapInfoRoot;

    #[method(name = "set_Info", args = 1)]
    pub fn set_info(self, value: crate::app::mapinforoot::MapInfoRoot) -> ();

    #[method(name = "get_UnitModel", args = 0)]
    pub fn get_unit_model(self) -> crate::app::unitmodel::UnitModel;

    #[method(name = "get_GodModel", args = 0)]
    pub fn get_god_model(self) -> crate::app::unitmodel::UnitModel;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_CellCenterPosition", args = 0)]
    pub fn get_cell_center_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_Direction", args = 0)]
    pub fn get_direction(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_BmapSize", args = 0)]
    pub fn get_bmap_size(self) -> i32;

    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_DisposX", args = 0)]
    pub fn get_dispos_x(self) -> i32;

    #[method(name = "set_DisposX", args = 1)]
    pub fn set_dispos_x(self, value: i32) -> ();

    #[method(name = "get_DisposZ", args = 0)]
    pub fn get_dispos_z(self) -> i32;

    #[method(name = "set_DisposZ", args = 1)]
    pub fn set_dispos_z(self, value: i32) -> ();

    #[method(name = "get_CenterX", args = 0)]
    pub fn get_center_x(self) -> i32;

    #[method(name = "get_CenterZ", args = 0)]
    pub fn get_center_z(self) -> i32;

    #[method(name = "get_Angle", args = 0)]
    pub fn get_angle(self) -> f32;

    #[method(name = "set_Angle", args = 1)]
    pub fn set_angle(self, value: f32) -> ();

    #[method(name = "get_DisposSound", args = 0)]
    pub fn get_dispos_sound(self) -> i32;

    #[method(name = "get_MoveType", args = 0)]
    pub fn get_move_type(self) -> crate::app::jobdata::JobData_MoveTypes;

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_LimitLevel", args = 0)]
    pub fn get_limit_level(self) -> i32;

    #[method(name = "get_Exp", args = 0)]
    pub fn get_exp(self) -> i32;

    #[method(name = "set_Exp", args = 1)]
    pub fn set_exp(self, value: i32) -> ();

    #[method(name = "get_SkillPoint", args = 0)]
    pub fn get_skill_point(self) -> i32;

    #[method(name = "set_SkillPoint", args = 1)]
    pub fn set_skill_point(self, value: i32) -> ();

    #[method(name = "get_Hp", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "set_Hp", args = 1)]
    pub fn set_hp(self, value: i32) -> ();

    #[method(name = "get_DisplayHp", args = 0)]
    pub fn get_display_hp(self) -> i32;

    #[method(name = "get_HpStockCount", args = 0)]
    pub fn get_hp_stock_count(self) -> i32;

    #[method(name = "get_HpStockCountMax", args = 0)]
    pub fn get_hp_stock_count_max(self) -> i32;

    #[method(name = "get_PlainHpStockCount", args = 0)]
    pub fn get_plain_hp_stock_count(self) -> i32;

    #[method(name = "set_PlainHpStockCount", args = 1)]
    pub fn set_plain_hp_stock_count(self, value: i32) -> ();

    #[method(name = "get_PlainHpStockCountMax", args = 0)]
    pub fn get_plain_hp_stock_count_max(self) -> i32;

    #[method(name = "get_ExtraHpStockCount", args = 0)]
    pub fn get_extra_hp_stock_count(self) -> i32;

    #[method(name = "get_ExtraHpStockCountMax", args = 0)]
    pub fn get_extra_hp_stock_count_max(self) -> i32;

    #[method(name = "get_BaseMovePower", args = 0)]
    pub fn get_base_move_power(self) -> i32;

    #[method(name = "get_ItemList", args = 0)]
    pub fn get_item_list(self) -> crate::app::unititemlist::UnitItemList;

    #[method(name = "get_GodUnit", args = 0)]
    pub fn get_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "get_ActualGodUnit", args = 0)]
    pub fn get_actual_god_unit(self) -> crate::app::godunit::GodUnit;

    #[method(name = "get_ExtraSight", args = 0)]
    pub fn get_extra_sight(self) -> i32;

    #[method(name = "set_ExtraSight", args = 1)]
    pub fn set_extra_sight(self, value: i32) -> ();

    #[method(name = "get_MoveDistance", args = 0)]
    pub fn get_move_distance(self) -> i32;

    #[method(name = "get_MaskSkillLock", args = 0)]
    pub fn get_mask_skill_lock(self) -> crate::system::object::Object;

    #[method(name = "get_MaskSkill", args = 0)]
    pub fn get_mask_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_EquipSkill", args = 0)]
    pub fn get_equip_skill_2(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_PrivateSkill", args = 0)]
    pub fn get_private_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_SupportedSkill", args = 0)]
    pub fn get_supported_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_ReceiveSkill", args = 0)]
    pub fn get_receive_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_EquipSkillPool", args = 0)]
    pub fn get_equip_skill_pool(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "get_LearnedJobSkill", args = 0)]
    pub fn get_learned_job_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_LearnedJobSkill", args = 1)]
    pub fn set_learned_job_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_OriginalAptitude", args = 0)]
    pub fn get_original_aptitude(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "get_Aptitude", args = 0)]
    pub fn get_aptitude(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "get_WeaponMask", args = 0)]
    pub fn get_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "get_SelectedWeaponMask", args = 0)]
    pub fn get_selected_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "get_EnhanceFactors", args = 0)]
    pub fn get_enhance_factors(self) -> crate::app::unitenhancefactors::UnitEnhanceFactors;

    #[method(name = "get_InternalLevel", args = 0)]
    pub fn get_internal_level(self) -> i32;

    #[method(name = "set_InternalLevel", args = 1)]
    pub fn set_internal_level(self, value: i32) -> ();

    #[method(name = "get_LastPickVoice", args = 0)]
    pub fn get_last_pick_voice(self) -> i8;

    #[method(name = "set_LastPickVoice", args = 1)]
    pub fn set_last_pick_voice(self, value: i8) -> ();

    #[method(name = "GetMaxStun", args = 0)]
    pub fn get_max_stun(self) -> i32;

    #[method(name = "GetStun", args = 0)]
    pub fn get_stun(self) -> i32;

    #[method(name = "get_EngageCount", args = 0)]
    pub fn get_engage_count(self) -> i32;

    #[method(name = "set_EngageCount", args = 1)]
    pub fn set_engage_count(self, value: i32) -> ();

    #[method(name = "PlaySetEngageCount", args = 2)]
    pub fn play_set_engage_count(
        self,
        engage_count: i32,
        target_unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "PlaySetEngageCount", args = 3)]
    pub fn play_set_engage_count_2(
        self,
        start: crate::unity_engine::vector3::Vector3,
        goal: crate::unity_engine::vector3::Vector3,
        count: i32,
    ) -> ();

    #[method(name = "GetEngageCountView", args = 0)]
    pub fn get_engage_count_view(self) -> i32;

    #[method(name = "SetEngageCountView", args = 1)]
    pub fn set_engage_count_view(self, count: i32) -> ();

    #[method(name = "GetEngageCountLimit", args = 0)]
    pub fn get_engage_count_limit(self) -> i32;

    #[method(name = "get_EngageTurn", args = 0)]
    pub fn get_engage_turn(self) -> i32;

    #[method(name = "set_EngageTurn", args = 1)]
    pub fn set_engage_turn(self, value: i32) -> ();

    #[method(name = "GetRemainingEngageTurn", args = 0)]
    pub fn get_remaining_engage_turn(self) -> i32;

    #[method(name = "GetEngageTurnLimit", args = 0)]
    pub fn get_engage_turn_limit(self) -> i32;

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "get_Ident", args = 0)]
    pub fn get_ident(self) -> i32;

    #[method(name = "set_Ident", args = 1)]
    pub fn set_ident(self, value: i32) -> ();

    #[method(name = "get_Record", args = 0)]
    pub fn get_record_2(self) -> crate::app::unitrecord::UnitRecord;

    #[method(name = "get_MapHistoryIndex", args = 0)]
    pub fn get_map_history_index(self) -> i32;

    #[method(name = "set_MapHistoryIndex", args = 1)]
    pub fn set_map_history_index(self, value: i32) -> ();

    #[method(name = "get_RelayPlayerIndex", args = 0)]
    pub fn get_relay_player_index(self) -> i32;

    #[method(name = "set_RelayPlayerIndex", args = 1)]
    pub fn set_relay_player_index(self, value: i32) -> ();

    #[method(name = "CreateImpl1", args = 4)]
    pub fn create_impl1(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        level: i32,
        random: crate::app::random_2::Random_2,
    ) -> ();

    #[method(name = "TryCopyEdit", args = 0)]
    pub fn try_copy_edit(self) -> ();

    #[method(name = "CreateImpl2", args = 0)]
    pub fn create_impl2(self) -> ();

    #[method(name = "SetDispos", args = 1)]
    pub fn set_dispos(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "SetDisposWeaponMask", args = 1)]
    pub fn set_dispos_weapon_mask(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "SetDisposSkill", args = 1)]
    pub fn set_dispos_skill(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "SetDisposGod", args = 1)]
    pub fn set_dispos_god(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "AddPersonItem", args = 1)]
    pub fn add_person_item(self, person: crate::app::persondata::PersonData) -> bool;

    #[method(name = "AddDisposItem", args = 1)]
    pub fn add_dispos_item(self, data: crate::app::disposdata::DisposData) -> bool;

    #[method(name = "SetDisposBelong", args = 1)]
    pub fn set_dispos_belong(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "SetDisposAi", args = 1)]
    pub fn set_dispos_ai(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "CreateGodStates", args = 1)]
    pub fn create_god_states(
        self,
        data: crate::app::disposdata::DisposData,
    ) -> ::unity2::Array<crate::app::godstate::GodState>;

    #[method(name = "GetGodState", args = 3)]
    pub fn get_god_state_2(
        self,
        data: crate::app::disposdata::DisposData,
        index: i32,
        prev_state: crate::app::godstate::GodState,
    ) -> crate::app::godstate::GodState;

    #[method(name = "DisposState2GodState", args = 1)]
    pub fn dispos_state2_god_state(
        self,
        dispos_state: crate::app::disposdata::DisposData_State,
    ) -> crate::app::godstate::GodState;

    #[method(name = "SetupForVision", args = 1)]
    pub fn setup_for_vision(self, original: crate::app::unit::Unit) -> ();

    #[method(name = "CreateForSummonImpl1", args = 3)]
    pub fn create_for_summon_impl1(
        self,
        person: crate::app::persondata::PersonData,
        original: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
    ) -> ();

    #[method(name = "SetupForSummon", args = 2)]
    pub fn setup_for_summon(
        self,
        owner: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
    ) -> ();

    #[method(name = "CreateImpl2ExcludeInternalLevel", args = 0)]
    pub fn create_impl2_exclude_internal_level(self) -> ();

    #[method(name = "SetupCapabilityForVision", args = 1)]
    pub fn setup_capability_for_vision(self, original: crate::app::unit::Unit) -> ();

    #[method(name = "GetSafePerson", args = 0)]
    pub fn get_safe_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "GetSafeJob", args = 0)]
    pub fn get_safe_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "CalculateAutoGrowCapability", args = 1)]
    pub fn calculate_auto_grow_capability(
        self,
        percents: crate::app::capabilityint::CapabilityInt,
    ) -> ();

    #[method(name = "GetMaxHpImpl", args = 1)]
    pub fn get_max_hp_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetStrImpl", args = 1)]
    pub fn get_str_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetTechImpl", args = 1)]
    pub fn get_tech_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetQuickImpl", args = 1)]
    pub fn get_quick_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetLuckImpl", args = 1)]
    pub fn get_luck_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetDefImpl", args = 1)]
    pub fn get_def_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetMagicImpl", args = 1)]
    pub fn get_magic_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetMdefImpl", args = 1)]
    pub fn get_mdef_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetPhysImpl", args = 1)]
    pub fn get_phys_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetSightImpl", args = 1)]
    pub fn get_sight_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetMovePowerImpl", args = 1)]
    pub fn get_move_power_impl(self, calc_enhance: bool) -> i32;

    #[method(name = "GetCapabilityImpl", args = 2)]
    pub fn get_capability_impl(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
        calc_enhance: bool,
    ) -> i32;

    #[method(name = "GetTotalLevel", args = 0)]
    pub fn get_total_level(self) -> i32;

    #[method(name = "GetOrder", args = 0)]
    pub fn get_order(self) -> i32;

    #[method(name = "CalcItemRange", args = 4)]
    pub fn calc_item_range(
        self,
        item: crate::app::itemdata::ItemData,
        range_i: i32,
        range_o: i32,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetItemGrowCapability", args = 1)]
    pub fn get_item_grow_capability(
        self,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "get_AttackImage", args = 0)]
    pub fn get_attack_image(self) -> crate::app::mapdeployattackimage::MapDeployAttackImage;

    #[method(name = "get_RodImage", args = 0)]
    pub fn get_rod_image(self) -> crate::app::mapdeployrodimage::MapDeployRodImage;

    #[method(name = "get_HealImage", args = 0)]
    pub fn get_heal_image(self) -> crate::app::mapdeployhealimage::MapDeployHealImage;

    #[method(name = "get_SupportImage", args = 0)]
    pub fn get_support_image(self) -> crate::app::mapdeploysupportimage::MapDeploySupportImage;

    #[method(name = "get_InterferenceImage", args = 0)]
    pub fn get_interference_image(
        self,
    ) -> crate::app::mapdeployinterferenceimage::MapDeployInterferenceImage;

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

    #[method(name = "get_SideType", args = 0)]
    pub fn get_side_type(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "set_SideType", args = 1)]
    pub fn set_side_type(self, value: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "UpdateImage", args = 1)]
    pub fn update_image(self, deploy: crate::app::mapdnagerdeploy::MapDnagerDeploy) -> ();

    #[method(name = "ClearImage", args = 0)]
    pub fn clear_image(self) -> ();

    #[method(name = "TryGetEngageMind", args = 1)]
    pub fn try_get_engage_mind(self, mind: crate::app::mapmind::MapMind_Type) -> bool;

    #[method(name = "GetEngageAttack", args = 0)]
    pub fn get_engage_attack(self) -> crate::app::skilldata::SkillData;

    #[method(name = "GetEngageSkills", args = 0)]
    pub fn get_engage_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetEngageSkill", args = 1)]
    pub fn get_engage_skill(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "GetSupportSkill", args = 0)]
    pub fn get_support_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "AddCannonSkill", args = 1)]
    pub fn add_cannon_skill(self, inspector: crate::app::cannoninspector::CannonInspector) -> ();

    #[method(name = "RemoveCannonSkill", args = 1)]
    pub fn remove_cannon_skill(self, inspector: crate::app::cannoninspector::CannonInspector)
        -> ();

    #[method(name = "GetCommandGiveSkills", args = 1)]
    pub fn get_command_give_skills(
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::skillarray::SkillArray;

    #[method(name = "AddCommandSkill", args = 1)]
    pub fn add_command_skill(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "RemoveCommandSkill", args = 1)]
    pub fn remove_command_skill(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "AddGiveSkill", args = 1)]
    pub fn add_give_skill(self, give: crate::app::skilldata::SkillData) -> ();

    #[method(name = "AddGiveSkill", args = 1)]
    pub fn add_give_skill_2(self, gives: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "get_BattleTemporary", args = 0)]
    pub fn get_battle_temporary(self) -> i32;

    #[method(name = "set_BattleTemporary", args = 1)]
    pub fn set_battle_temporary(self, value: i32) -> ();

    #[method(name = "IsExistDie", args = 0)]
    pub fn is_exist_die(self) -> bool;

    #[method(name = "UpdateMoveDistance", args = 0)]
    pub fn update_move_distance(self) -> ();

    #[method(name = "UpdateMoveDistance", args = 2)]
    pub fn update_move_distance_2(self, x: i32, z: i32) -> ();

    #[method(name = "UpdateMoveDistance", args = 3)]
    pub fn update_move_distance_3(
        self,
        route: crate::app::maproute::MapRoute,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "ClearMoveDistance", args = 0)]
    pub fn clear_move_distance(self) -> ();

    #[method(name = "CanGrowUp", args = 1)]
    pub fn can_grow_up(
        self,
        r#type: crate::app::capabilitydefinition::CapabilityDefinition_Type,
    ) -> bool;

    #[method(name = "GetSimplePower", args = 0)]
    pub fn get_simple_power(self) -> i32;

    #[method(name = "GetSimplePhysicalPower", args = 0)]
    pub fn get_simple_physical_power(self) -> i32;

    #[method(name = "GetSimpleMagicPower", args = 0)]
    pub fn get_simple_magic_power(self) -> i32;

    #[method(name = "GetSimplePhysicalDefense", args = 0)]
    pub fn get_simple_physical_defense(self) -> i32;

    #[method(name = "GetSimpleMagicDefense", args = 0)]
    pub fn get_simple_magic_defense(self) -> i32;

    #[method(name = "GetSimpleHit", args = 0)]
    pub fn get_simple_hit(self) -> i32;

    #[method(name = "GetSimpleAvoid", args = 0)]
    pub fn get_simple_avoid(self) -> i32;

    #[method(name = "GetSimpleCritical", args = 0)]
    pub fn get_simple_critical(self) -> i32;

    #[method(name = "GetSimpleSecure", args = 0)]
    pub fn get_simple_secure(self) -> i32;

    #[method(name = "GetSimpleContinuous", args = 0)]
    pub fn get_simple_continuous(self) -> i32;

    #[method(name = "ResetParam", args = 0)]
    pub fn reset_param(self) -> ();

    #[method(name = "get_FortuneSeed", args = 0)]
    pub fn get_fortune_seed(self) -> u32;

    #[method(name = "set_FortuneSeed", args = 1)]
    pub fn set_fortune_seed(self, value: u32) -> ();

    #[method(name = "get_FortuneTarget", args = 0)]
    pub fn get_fortune_target(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_FortuneTarget", args = 1)]
    pub fn set_fortune_target(self, value: crate::app::persondata::PersonData) -> ();

    #[method(name = "GetFortuneRandomValue", args = 1)]
    pub fn get_fortune_random_value(self, update: bool) -> i32;

    #[method(name = "GetWeaponLevel", args = 2)]
    pub fn get_weapon_level(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
        calc_enhance: bool,
    ) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "GetSkillEquip", args = 2)]
    pub fn get_skill_equip(
        self,
        skill: crate::app::skilldata::SkillData,
        index: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetEngageEquip", args = 2)]
    pub fn get_engage_equip(
        self,
        skill: crate::app::skilldata::SkillData,
        target: crate::app::unit::Unit,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetEffectPosition", args = 0)]
    pub fn get_effect_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CanStayWithoutGod", args = 0)]
    pub fn can_stay_without_god(self) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-unit")]
impl Unit {
    pub fn new(use_image: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitMethods>::ctor(this, use_image);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_DisposItemsForSelectedWeapon.md")))]
#[::unity2::class(namespace = "App", name = "Unit.DisposItemsForSelectedWeapon")]
#[parent(crate::app::unit::Unit_ItemsForSelectedWeapon)]
pub struct Unit_DisposItemsForSelectedWeapon {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::disposdata::DisposData,
}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_DisposItemsForSelectedWeapon {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::disposdata::DisposData) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_DisposItemsForSelectedWeapon {
    pub fn new(data: crate::app::disposdata::DisposData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_DisposItemsForSelectedWeapon),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_DisposItemsForSelectedWeaponMethods>::ctor(this, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_ChartItemsForSelectedWeapon.md")))]
#[::unity2::class(namespace = "App", name = "Unit.ChartItemsForSelectedWeapon")]
#[parent(crate::app::unit::Unit_ItemsForSelectedWeapon)]
pub struct Unit_ChartItemsForSelectedWeapon {
    #[rename(name = "m_ChartItems")]
    pub m_chart_items: ::unity2::Array<crate::app::chartdata::ChartData_Item>,
}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_ChartItemsForSelectedWeapon {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, chart_items: ::unity2::Array<crate::app::chartdata::ChartData_Item>) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_ChartItemsForSelectedWeapon {
    pub fn new(chart_items: ::unity2::Array<crate::app::chartdata::ChartData_Item>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_ChartItemsForSelectedWeapon),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_ChartItemsForSelectedWeaponMethods>::ctor(this, chart_items);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_ChangeValue.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Unit_ChangeValue {
    pub m_value: u8,
    pub m_display: u8,
}

impl ::unity2::ClassIdentity for Unit_ChangeValue {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Unit.ChangeValue";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Unit_ChangeValue {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-unit")]
#[::unity2::methods(value)]
impl Unit_ChangeValue {
    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> u8;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: u8) -> ();

    #[method(name = "get_Display", args = 0)]
    pub fn get_display(self) -> u8;

    #[method(name = "Change", args = 1)]
    pub fn change(self, value: i32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "IsChanging", args = 0)]
    pub fn is_changing(self) -> bool;

    #[method(name = "Instant", args = 0)]
    pub fn instant(self) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(v: crate::app::unit::Unit_ChangeValue) -> u8;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_GuardType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Unit_GuardType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Unit_GuardType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Unit.GuardType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Unit_GuardType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Unit_GuardType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn chain_guard() -> Self {
        Self { value: 1 }
    }

    pub fn dual_guard() -> Self {
        Self { value: 2 }
    }

    pub fn not_enough_hp() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_FuncUnitItem.md")))]
#[::unity2::class(namespace = "App", name = "Unit.FuncUnitItem")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Unit_FuncUnitItem {}

#[cfg(feature = "app-unit")]
#[::unity2::methods]
impl Unit_FuncUnitItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem) -> ();
}

#[cfg(feature = "app-unit")]
impl Unit_FuncUnitItem {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Unit_FuncUnitItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnit_FuncUnitItemMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unit/Unit_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Unit_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Unit_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Unit.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Unit_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Unit_Status {
    pub fn fixed() -> Self {
        Self { value: 1 }
    }

    pub fn move_not_allow() -> Self {
        Self { value: 2 }
    }

    pub fn must_sortie() -> Self {
        Self { value: 4 }
    }

    pub fn never_sortie() -> Self {
        Self { value: 8 }
    }

    pub fn dont_pos_change() -> Self {
        Self { value: 16 }
    }

    pub fn danger_showing() -> Self {
        Self { value: 32 }
    }

    pub fn chain_guard() -> Self {
        Self { value: 64 }
    }

    pub fn dual_guard() -> Self {
        Self { value: 128 }
    }

    pub fn sortie() -> Self {
        Self { value: 256 }
    }

    pub fn dead() -> Self {
        Self { value: 512 }
    }

    pub fn escape_here() -> Self {
        Self { value: 1024 }
    }

    pub fn died_here() -> Self {
        Self { value: 2048 }
    }

    pub fn join_here() -> Self {
        Self { value: 4096 }
    }

    pub fn pure_hide() -> Self {
        Self { value: 8192 }
    }

    pub fn dispos_hide() -> Self {
        Self { value: 16384 }
    }

    pub fn view_out() -> Self {
        Self { value: 32768 }
    }

    pub fn under_roof() -> Self {
        Self { value: 65536 }
    }

    pub fn locked_update() -> Self {
        Self { value: 131072 }
    }

    pub fn removing() -> Self {
        Self { value: 262144 }
    }

    pub fn remagicing() -> Self {
        Self { value: 524288 }
    }

    pub fn rerewarping() -> Self {
        Self { value: 1048576 }
    }

    pub fn guest() -> Self {
        Self { value: 2097152 }
    }

    pub fn dispos_guset() -> Self {
        Self { value: 4194304 }
    }

    pub fn engaging() -> Self {
        Self { value: 8388608 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 16777216 }
    }

    pub fn engage_linked() -> Self {
        Self { value: 33554432 }
    }

    pub fn engage_attacked() -> Self {
        Self { value: 67108864 }
    }

    pub fn vision() -> Self {
        Self { value: 134217728 }
    }

    pub fn exist_dead() -> Self {
        Self { value: 268435456 }
    }

    pub fn continued() -> Self {
        Self { value: 536870912 }
    }

    pub fn defect() -> Self {
        Self { value: 1073741824 }
    }

    pub fn bow_cannon() -> Self {
        Self { value: 0 }
    }

    pub fn magic_cannon() -> Self {
        Self { value: 0 }
    }

    pub fn fire_cannon() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_whole_skill() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_equip_skill() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_equip_enhance() -> Self {
        Self { value: 0 }
    }

    pub fn relay_leave() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_immortal() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_god_unit() -> Self {
        Self { value: 0 }
    }

    pub fn igonre_map_enhance() -> Self {
        Self { value: 0 }
    }

    pub fn before_sortied() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_supported_skill() -> Self {
        Self { value: 0 }
    }

    pub fn summon() -> Self {
        Self { value: 0 }
    }

    pub fn lockon() -> Self {
        Self { value: 0 }
    }

    pub fn reacted() -> Self {
        Self { value: 0 }
    }

    pub fn hold_hp() -> Self {
        Self { value: 0 }
    }

    pub fn ignore_map_history() -> Self {
        Self { value: 0 }
    }

    pub fn change_engaged() -> Self {
        Self { value: 0 }
    }

    pub fn locked_support() -> Self {
        Self { value: 0 }
    }

    pub fn hide_image() -> Self {
        Self { value: 8192 }
    }

    pub fn hide_render() -> Self {
        Self { value: 122880 }
    }

    pub fn not_target() -> Self {
        Self { value: 65536 }
    }

    pub fn init_map_begin() -> Self {
        Self { value: 536871196 }
    }

    pub fn init_phase_begin() -> Self {
        Self { value: 192 }
    }

    pub fn init_phase_end() -> Self {
        Self { value: 52166657 }
    }

    pub fn init_map_end() -> Self {
        Self { value: 1201660413 }
    }

    pub fn dead_mask() -> Self {
        Self { value: 268438016 }
    }

    pub fn guard_mask() -> Self {
        Self { value: 192 }
    }

    pub fn engage_mask() -> Self {
        Self { value: 125829248 }
    }

    pub fn fixed_mask() -> Self {
        Self { value: 193 }
    }

    pub fn cannon_mask() -> Self {
        Self { value: 0 }
    }

    pub fn reaction_mask() -> Self {
        Self { value: 1835008 }
    }

    pub fn save_mask() -> Self {
        Self { value: -1 }
    }
}
