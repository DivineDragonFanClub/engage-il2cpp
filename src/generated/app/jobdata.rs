
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData_MoveTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobData_MoveTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobData_MoveTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobData.MoveTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobData_MoveTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobData_MoveTypes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn foot() -> Self {
        Self { value: 1 }
    }

    pub fn horse() -> Self {
        Self { value: 2 }
    }

    pub fn fly() -> Self {
        Self { value: 3 }
    }

    pub fn dragon() -> Self {
        Self { value: 4 }
    }

    pub fn pad() -> Self {
        Self { value: 5 }
    }

    pub fn num() -> Self {
        Self { value: 6 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData.md")))]
#[::unity2::class(namespace = "App", name = "JobData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: jobdata :: JobData >)]
pub struct JobData {
    #[static_field]
    #[rename(name = "MaxHighJob")]
    pub max_high_job: i32,
    #[static_field]
    #[rename(name = "JidMaleSuffix")]
    pub jid_male_suffix: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "JidFemaleSuffix")]
    pub jid_female_suffix: ::unity2::Il2CppString,
    #[rename(name = "Weapons")]
    pub weapons: ::unity2::Array<i8>,
    #[rename(name = "MaxWeaponLevels")]
    pub max_weapon_levels: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "WeaponLevels")]
    pub weapon_levels: ::unity2::Array<crate::app::weaponlevel::WeaponLevel_Kind>,
    #[rename(name = "WeaponLevelPlusMask")]
    pub weapon_level_plus_mask: crate::app::weaponmask::WeaponMask,
    #[rename(name = "HighJobs")]
    pub high_jobs: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-jobdata")]
#[::unity2::methods]
impl JobData {
    #[method(name = "GetMoveTypeName", args = 1)]
    pub fn get_move_type_name(
        r#type: crate::app::jobdata::JobData_MoveTypes,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Jid", args = 1)]
    pub fn set_jid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Aid", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Aid", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnitIconID_M", args = 0)]
    pub fn get_unit_icon_id_m(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitIconID_M", args = 1)]
    pub fn set_unit_icon_id_m(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnitIconID_F", args = 0)]
    pub fn get_unit_icon_id_f(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitIconID_F", args = 1)]
    pub fn set_unit_icon_id_f(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnitIconWeaponID", args = 0)]
    pub fn get_unit_icon_weapon_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitIconWeaponID", args = 1)]
    pub fn set_unit_icon_weapon_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Rank", args = 0)]
    pub fn get_rank(self) -> i8;

    #[method(name = "set_Rank", args = 1)]
    pub fn set_rank(self, value: i8) -> ();

    #[method(name = "get_StyleName", args = 0)]
    pub fn get_style_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StyleName", args = 1)]
    pub fn set_style_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MoveType", args = 0)]
    pub fn get_move_type(self) -> crate::app::jobdata::JobData_MoveTypes;

    #[method(name = "set_MoveType", args = 1)]
    pub fn set_move_type(self, value: crate::app::jobdata::JobData_MoveTypes) -> ();

    #[method(name = "get_StepFrame", args = 0)]
    pub fn get_step_frame(self) -> i32;

    #[method(name = "set_StepFrame", args = 1)]
    pub fn set_step_frame(self, value: i32) -> ();

    #[method(name = "get_MaxLevel", args = 0)]
    pub fn get_max_level(self) -> u8;

    #[method(name = "set_MaxLevel", args = 1)]
    pub fn set_max_level(self, value: u8) -> ();

    #[method(name = "get_InternalLevel", args = 0)]
    pub fn get_internal_level(self) -> i8;

    #[method(name = "set_InternalLevel", args = 1)]
    pub fn set_internal_level(self, value: i8) -> ();

    #[method(name = "get_Sort", args = 0)]
    pub fn get_sort(self) -> u16;

    #[method(name = "set_Sort", args = 1)]
    pub fn set_sort(self, value: u16) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::jobdata::JobData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::jobdata::JobData_FlagField) -> ();

    #[method(name = "get_CCItems", args = 0)]
    pub fn get_cc_items(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_CCItems", args = 1)]
    pub fn set_cc_items(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_UniqueItems", args = 0)]
    pub fn get_unique_items(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_UniqueItems", args = 1)]
    pub fn set_unique_items(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = "set_Style", args = 1)]
    pub fn set_style(self, value: crate::app::battlestyle::BattleStyle_Types) -> ();

    #[method(name = "get_WeaponNone", args = 0)]
    pub fn get_weapon_none(self) -> i8;

    #[method(name = "set_WeaponNone", args = 1)]
    pub fn set_weapon_none(self, value: i8) -> ();

    #[method(name = "get_WeaponSword", args = 0)]
    pub fn get_weapon_sword(self) -> i8;

    #[method(name = "set_WeaponSword", args = 1)]
    pub fn set_weapon_sword(self, value: i8) -> ();

    #[method(name = "get_WeaponLance", args = 0)]
    pub fn get_weapon_lance(self) -> i8;

    #[method(name = "set_WeaponLance", args = 1)]
    pub fn set_weapon_lance(self, value: i8) -> ();

    #[method(name = "get_WeaponAxe", args = 0)]
    pub fn get_weapon_axe(self) -> i8;

    #[method(name = "set_WeaponAxe", args = 1)]
    pub fn set_weapon_axe(self, value: i8) -> ();

    #[method(name = "get_WeaponBow", args = 0)]
    pub fn get_weapon_bow(self) -> i8;

    #[method(name = "set_WeaponBow", args = 1)]
    pub fn set_weapon_bow(self, value: i8) -> ();

    #[method(name = "get_WeaponDagger", args = 0)]
    pub fn get_weapon_dagger(self) -> i8;

    #[method(name = "set_WeaponDagger", args = 1)]
    pub fn set_weapon_dagger(self, value: i8) -> ();

    #[method(name = "get_WeaponMagic", args = 0)]
    pub fn get_weapon_magic(self) -> i8;

    #[method(name = "set_WeaponMagic", args = 1)]
    pub fn set_weapon_magic(self, value: i8) -> ();

    #[method(name = "get_WeaponRod", args = 0)]
    pub fn get_weapon_rod(self) -> i8;

    #[method(name = "set_WeaponRod", args = 1)]
    pub fn set_weapon_rod(self, value: i8) -> ();

    #[method(name = "get_WeaponFist", args = 0)]
    pub fn get_weapon_fist(self) -> i8;

    #[method(name = "set_WeaponFist", args = 1)]
    pub fn set_weapon_fist(self, value: i8) -> ();

    #[method(name = "get_WeaponSpecial", args = 0)]
    pub fn get_weapon_special(self) -> i8;

    #[method(name = "set_WeaponSpecial", args = 1)]
    pub fn set_weapon_special(self, value: i8) -> ();

    #[method(name = "get_WeaponTool", args = 0)]
    pub fn get_weapon_tool(self) -> i8;

    #[method(name = "set_WeaponTool", args = 1)]
    pub fn set_weapon_tool(self, value: i8) -> ();

    #[method(name = "get_MaxWeaponLevelNone", args = 0)]
    pub fn get_max_weapon_level_none(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelNone", args = 1)]
    pub fn set_max_weapon_level_none(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelSword", args = 0)]
    pub fn get_max_weapon_level_sword(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelSword", args = 1)]
    pub fn set_max_weapon_level_sword(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelLance", args = 0)]
    pub fn get_max_weapon_level_lance(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelLance", args = 1)]
    pub fn set_max_weapon_level_lance(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelAxe", args = 0)]
    pub fn get_max_weapon_level_axe(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelAxe", args = 1)]
    pub fn set_max_weapon_level_axe(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelBow", args = 0)]
    pub fn get_max_weapon_level_bow(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelBow", args = 1)]
    pub fn set_max_weapon_level_bow(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelMagic", args = 0)]
    pub fn get_max_weapon_level_magic(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelMagic", args = 1)]
    pub fn set_max_weapon_level_magic(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelRod", args = 0)]
    pub fn get_max_weapon_level_rod(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelRod", args = 1)]
    pub fn set_max_weapon_level_rod(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelDagger", args = 0)]
    pub fn get_max_weapon_level_dagger(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelDagger", args = 1)]
    pub fn set_max_weapon_level_dagger(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelFist", args = 0)]
    pub fn get_max_weapon_level_fist(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelFist", args = 1)]
    pub fn set_max_weapon_level_fist(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxWeaponLevelSpecial", args = 0)]
    pub fn get_max_weapon_level_special(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MaxWeaponLevelSpecial", args = 1)]
    pub fn set_max_weapon_level_special(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HighJob1", args = 0)]
    pub fn get_high_job1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HighJob1", args = 1)]
    pub fn set_high_job1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HighJob2", args = 0)]
    pub fn get_high_job2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HighJob2", args = 1)]
    pub fn set_high_job2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LowJob", args = 0)]
    pub fn get_low_job(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LowJob", args = 1)]
    pub fn set_low_job(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Base", args = 0)]
    pub fn get_base(self) -> crate::app::capability::Capability;

    #[method(name = "set_Base", args = 1)]
    pub fn set_base(self, value: crate::app::capability::Capability) -> ();

    #[method(name = "get_Limit", args = 0)]
    pub fn get_limit(self) -> crate::app::capability::Capability;

    #[method(name = "set_Limit", args = 1)]
    pub fn set_limit(self, value: crate::app::capability::Capability) -> ();

    #[method(name = "get_BaseGrow", args = 0)]
    pub fn get_base_grow(self) -> crate::app::capability::Capability;

    #[method(name = "set_BaseGrow", args = 1)]
    pub fn set_base_grow(self, value: crate::app::capability::Capability) -> ();

    #[method(name = "get_DiffGrow", args = 0)]
    pub fn get_diff_grow(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_DiffGrow", args = 1)]
    pub fn set_diff_grow(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_DiffGrowNormal", args = 0)]
    pub fn get_diff_grow_normal(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_DiffGrowNormal", args = 1)]
    pub fn set_diff_grow_normal(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_DiffGrowHard", args = 0)]
    pub fn get_diff_grow_hard(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_DiffGrowHard", args = 1)]
    pub fn set_diff_grow_hard(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_DiffGrowLunatic", args = 0)]
    pub fn get_diff_grow_lunatic(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_DiffGrowLunatic", args = 1)]
    pub fn set_diff_grow_lunatic(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_ShortName", args = 0)]
    pub fn get_short_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ShortName", args = 1)]
    pub fn set_short_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Skills", args = 0)]
    pub fn get_skills(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Skills", args = 1)]
    pub fn set_skills(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_LearningSkill", args = 0)]
    pub fn get_learning_skill(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LearningSkill", args = 1)]
    pub fn set_learning_skill(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LunaticSkill", args = 0)]
    pub fn get_lunatic_skill(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LunaticSkill", args = 1)]
    pub fn set_lunatic_skill(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attrs", args = 0)]
    pub fn get_attrs(self) -> crate::app::skilldata::SkillData_Attrs;

    #[method(name = "set_Attrs", args = 1)]
    pub fn set_attrs(self, value: crate::app::skilldata::SkillData_Attrs) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "GetPrefixlessJid", args = 0)]
    pub fn get_prefixless_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "GetUnitIconID", args = 1)]
    pub fn get_unit_icon_id(self, is_female: bool) -> ::unity2::Il2CppString;

    #[method(name = "IsHigh", args = 0)]
    pub fn is_high(self) -> bool;

    #[method(name = "IsLow", args = 0)]
    pub fn is_low(self) -> bool;

    #[method(name = "IsFly", args = 0)]
    pub fn is_fly(self) -> bool;

    #[method(name = "IsRider", args = 0)]
    pub fn is_rider(self) -> bool;

    #[method(name = "IsDownload", args = 0)]
    pub fn is_download(self) -> bool;

    #[method(name = "IsUnknown", args = 0)]
    pub fn is_unknown(self) -> bool;

    #[method(name = "HasHighJobs", args = 0)]
    pub fn has_high_jobs(self) -> bool;

    #[method(name = "GetHighJobs", args = 0)]
    pub fn get_high_jobs(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "GetLowJobs", args = 0)]
    pub fn get_low_jobs(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::jobdata::JobData>;

    #[method(name = "IsEnchant", args = 0)]
    pub fn is_enchant(self) -> bool;

    #[method(name = "IsGunner", args = 0)]
    pub fn is_gunner(self) -> bool;

    #[method(name = "GetWeaponMask", args = 2)]
    pub fn get_weapon_mask(
        self,
        mask: crate::app::weaponmask::WeaponMask,
        selected: crate::app::weaponmask::WeaponMask,
    ) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "GetWeaponMask", args = 0)]
    pub fn get_weapon_mask_2(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "GetSelectableWeaponMask", args = 1)]
    pub fn get_selectable_weapon_mask(
        self,
        required_select_count: i32,
    ) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "GetEquipableWeaponKinds", args = 0)]
    pub fn get_equipable_weapon_kinds(
        self,
    ) -> ::unity2::Array<crate::app::itemdata::ItemData_Kinds>;

    #[method(name = "GetMaxLevelWeapons", args = 2)]
    pub fn get_max_level_weapons(
        self,
        weapon_mask: crate::app::weaponmask::WeaponMask,
        original_aptitude: crate::app::weaponmask::WeaponMask,
    ) -> ::unity2::Array<crate::app::itemdata::ItemData_Kinds>;

    #[method(name = "IsEquipable", args = 1)]
    pub fn is_equipable(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "GetMaxWeaponLevel", args = 1)]
    pub fn get_max_weapon_level(self, index: i32) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "GetMaxWeaponLevel", args = 2)]
    pub fn get_max_weapon_level_2(
        self,
        index: i32,
        original_aptitude: crate::app::weaponmask::WeaponMask,
    ) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "GetLevelPlusWeaponMask", args = 0)]
    pub fn get_level_plus_weapon_mask(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "GetLearnJobSkillLevel", args = 0)]
    pub fn get_learn_job_skill_level(self) -> i32;

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(
        stream: crate::app::stream_2::Stream_2,
        job: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> crate::app::jobdata::JobData;

    #[method(name = "TrySerialize", args = 2)]
    pub fn try_serialize(
        stream: crate::app::stream_2::Stream_2,
        job: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "TryDeserialize", args = 1)]
    pub fn try_deserialize(stream: crate::app::stream_2::Stream_2) -> crate::app::jobdata::JobData;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_MaskSkill", args = 0)]
    pub fn get_mask_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "set_MaskSkill", args = 1)]
    pub fn set_mask_skill(self, value: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "DbgVerify", args = 0)]
    pub fn dbg_verify(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-jobdata")]
impl JobData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobData),
                ::core::stringify!(new),
            )
        });
        <Self as IJobDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobData_Flags {
    pub fn can_cc() -> Self {
        Self { value: 1 }
    }

    pub fn anyone_cc() -> Self {
        Self { value: 2 }
    }

    pub fn female_only() -> Self {
        Self { value: 4 }
    }

    pub fn encount_map() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "JobData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: jobdata :: JobData_Flags >)]
pub struct JobData_FlagField {}

#[cfg(feature = "app-jobdata")]
#[::unity2::methods]
impl JobData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::jobdata::JobData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::jobdata::JobData_Flags) -> i32;
}

#[cfg(feature = "app-jobdata")]
impl JobData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IJobData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::jobdata::JobData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IJobData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData_WeaponValues.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobData_WeaponValues {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobData_WeaponValues {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobData.WeaponValues";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobData_WeaponValues {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobData_WeaponValues {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn equippable() -> Self {
        Self { value: 1 }
    }

    pub fn selectable1() -> Self {
        Self { value: 2 }
    }

    pub fn selectable2() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobdata/JobData_Ranks.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobData_Ranks {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobData_Ranks {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobData.Ranks";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobData_Ranks {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobData_Ranks {
    pub fn low() -> Self {
        Self { value: 0 }
    }

    pub fn high() -> Self {
        Self { value: 1 }
    }
}
