
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_Timing.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PersonData_Timing {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PersonData_Timing {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PersonData.Timing";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PersonData_Timing {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PersonData_Timing {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn begin() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }

    pub fn chapter() -> Self {
        Self { value: 3 }
    }

    pub fn eternal() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_Colors.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PersonData_Colors {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PersonData_Colors {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PersonData.Colors";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PersonData_Colors {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PersonData_Colors {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn red() -> Self {
        Self { value: 1 }
    }

    pub fn green() -> Self {
        Self { value: 2 }
    }

    pub fn blue() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "PersonData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: persondata :: PersonData_Flags >)]
pub struct PersonData_FlagField {}

#[cfg(feature = "app-persondata")]
#[::unity2::methods]
impl PersonData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::persondata::PersonData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::persondata::PersonData_Flags) -> i32;
}

#[cfg(feature = "app-persondata")]
impl PersonData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersonData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IPersonData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::persondata::PersonData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersonData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IPersonData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData.md")))]
#[::unity2::class(namespace = "App", name = "PersonData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: persondata :: PersonData >)]
pub struct PersonData {
    #[static_field]
    #[rename(name = "s_Veyre")]
    pub s_veyre: crate::app::persondata::PersonData,
}

#[cfg(feature = "app-persondata")]
#[::unity2::methods]
impl PersonData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Jid", args = 1)]
    pub fn set_jid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Fid", args = 0)]
    pub fn get_fid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Fid", args = 1)]
    pub fn set_fid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Aid", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Aid", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Die", args = 0)]
    pub fn get_die(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Die", args = 1)]
    pub fn set_die(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Belong", args = 0)]
    pub fn get_belong(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Belong", args = 1)]
    pub fn set_belong(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnitIconID", args = 0)]
    pub fn get_unit_icon_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitIconID", args = 1)]
    pub fn set_unit_icon_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Age", args = 0)]
    pub fn get_age(self) -> i16;

    #[method(name = "set_Age", args = 1)]
    pub fn set_age(self, value: i16) -> ();

    #[method(name = "get_BirthMonth", args = 0)]
    pub fn get_birth_month(self) -> u8;

    #[method(name = "set_BirthMonth", args = 1)]
    pub fn set_birth_month(self, value: u8) -> ();

    #[method(name = "get_BirthDay", args = 0)]
    pub fn get_birth_day(self) -> u8;

    #[method(name = "set_BirthDay", args = 1)]
    pub fn set_birth_day(self, value: u8) -> ();

    #[method(name = "get_Gender", args = 0)]
    pub fn get_gender(self) -> crate::app::gender::Gender;

    #[method(name = "set_Gender", args = 1)]
    pub fn set_gender(self, value: crate::app::gender::Gender) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> u8;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: u8) -> ();

    #[method(name = "get_InternalLevel", args = 0)]
    pub fn get_internal_level(self) -> i8;

    #[method(name = "set_InternalLevel", args = 1)]
    pub fn set_internal_level(self, value: i8) -> ();

    #[method(name = "get_AutoGrowOffsetN", args = 0)]
    pub fn get_auto_grow_offset_n(self) -> i8;

    #[method(name = "set_AutoGrowOffsetN", args = 1)]
    pub fn set_auto_grow_offset_n(self, value: i8) -> ();

    #[method(name = "get_AutoGrowOffsetH", args = 0)]
    pub fn get_auto_grow_offset_h(self) -> i8;

    #[method(name = "set_AutoGrowOffsetH", args = 1)]
    pub fn set_auto_grow_offset_h(self, value: i8) -> ();

    #[method(name = "get_AutoGrowOffsetL", args = 0)]
    pub fn get_auto_grow_offset_l(self) -> i8;

    #[method(name = "set_AutoGrowOffsetL", args = 1)]
    pub fn set_auto_grow_offset_l(self, value: i8) -> ();

    #[method(name = "get_AssetForce", args = 0)]
    pub fn get_asset_force(self) -> crate::app::force::Force_Type;

    #[method(name = "set_AssetForce", args = 1)]
    pub fn set_asset_force(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_SupportCategory", args = 0)]
    pub fn get_support_category(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SupportCategory", args = 1)]
    pub fn set_support_category(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SkillPoint", args = 0)]
    pub fn get_skill_point(self) -> i32;

    #[method(name = "set_SkillPoint", args = 1)]
    pub fn set_skill_point(self, value: i32) -> ();

    #[method(name = "get_BmapSize", args = 0)]
    pub fn get_bmap_size(self) -> u8;

    #[method(name = "set_BmapSize", args = 1)]
    pub fn set_bmap_size(self, value: u8) -> ();

    #[method(name = "get_Items", args = 0)]
    pub fn get_items(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Items", args = 1)]
    pub fn set_items(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_DropItem", args = 0)]
    pub fn get_drop_item(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DropItem", args = 1)]
    pub fn set_drop_item(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DropRatio", args = 0)]
    pub fn get_drop_ratio(self) -> f32;

    #[method(name = "set_DropRatio", args = 1)]
    pub fn set_drop_ratio(self, value: f32) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::persondata::PersonData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::persondata::PersonData_FlagField) -> ();

    #[method(name = "get_Aptitude", args = 0)]
    pub fn get_aptitude(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_Aptitude", args = 1)]
    pub fn set_aptitude(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_SubAptitude", args = 0)]
    pub fn get_sub_aptitude(self) -> crate::app::weaponmask::WeaponMask;

    #[method(name = "set_SubAptitude", args = 1)]
    pub fn set_sub_aptitude(self, value: crate::app::weaponmask::WeaponMask) -> ();

    #[method(name = "get_OffsetN", args = 0)]
    pub fn get_offset_n(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_OffsetN", args = 1)]
    pub fn set_offset_n(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_OffsetH", args = 0)]
    pub fn get_offset_h(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_OffsetH", args = 1)]
    pub fn set_offset_h(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_OffsetL", args = 0)]
    pub fn get_offset_l(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_OffsetL", args = 1)]
    pub fn set_offset_l(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_Limit", args = 0)]
    pub fn get_limit(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_Limit", args = 1)]
    pub fn set_limit(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_Grow", args = 0)]
    pub fn get_grow(self) -> crate::app::capability::Capability;

    #[method(name = "set_Grow", args = 1)]
    pub fn set_grow(self, value: crate::app::capability::Capability) -> ();

    #[method(name = "get_CommonSids", args = 0)]
    pub fn get_common_sids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_CommonSids", args = 1)]
    pub fn set_common_sids(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_NormalSids", args = 0)]
    pub fn get_normal_sids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_NormalSids", args = 1)]
    pub fn set_normal_sids(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_HardSids", args = 0)]
    pub fn get_hard_sids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_HardSids", args = 1)]
    pub fn set_hard_sids(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_LunaticSids", args = 0)]
    pub fn get_lunatic_sids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_LunaticSids", args = 1)]
    pub fn set_lunatic_sids(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_EngageSid", args = 0)]
    pub fn get_engage_sid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngageSid", args = 1)]
    pub fn set_engage_sid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TalkPauseDelayMin", args = 0)]
    pub fn get_talk_pause_delay_min(self) -> f32;

    #[method(name = "set_TalkPauseDelayMin", args = 1)]
    pub fn set_talk_pause_delay_min(self, value: f32) -> ();

    #[method(name = "get_TalkPauseDelayMax", args = 0)]
    pub fn get_talk_pause_delay_max(self) -> f32;

    #[method(name = "set_TalkPauseDelayMax", args = 1)]
    pub fn set_talk_pause_delay_max(self, value: f32) -> ();

    #[method(name = "get_TalkPauseSpeed", args = 0)]
    pub fn get_talk_pause_speed(self) -> f32;

    #[method(name = "set_TalkPauseSpeed", args = 1)]
    pub fn set_talk_pause_speed(self, value: f32) -> ();

    #[method(name = "get_CombatBgm", args = 0)]
    pub fn get_combat_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CombatBgm", args = 1)]
    pub fn set_combat_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AsciiName", args = 0)]
    pub fn get_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AsciiName", args = 1)]
    pub fn set_ascii_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LinkGod", args = 0)]
    pub fn get_link_god(self) -> crate::app::goddata::GodData;

    #[method(name = "set_LinkGod", args = 1)]
    pub fn set_link_god(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "get_Attrs", args = 0)]
    pub fn get_attrs(self) -> crate::app::skilldata::SkillData_Attrs;

    #[method(name = "set_Attrs", args = 1)]
    pub fn set_attrs(self, value: crate::app::skilldata::SkillData_Attrs) -> ();

    #[method(name = "get_ExistDieCid", args = 0)]
    pub fn get_exist_die_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ExistDieCid", args = 1)]
    pub fn set_exist_die_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ExistDieTiming", args = 0)]
    pub fn get_exist_die_timing(self) -> crate::app::persondata::PersonData_Timing;

    #[method(name = "set_ExistDieTiming", args = 1)]
    pub fn set_exist_die_timing(self, value: crate::app::persondata::PersonData_Timing) -> ();

    #[method(name = "get_Hometown", args = 0)]
    pub fn get_hometown(self) -> crate::app::persondata::PersonData_Country;

    #[method(name = "set_Hometown", args = 1)]
    pub fn set_hometown(self, value: crate::app::persondata::PersonData_Country) -> ();

    #[method(name = "get_NetRankingIndex", args = 0)]
    pub fn get_net_ranking_index(self) -> u8;

    #[method(name = "set_NetRankingIndex", args = 1)]
    pub fn set_net_ranking_index(self, value: u8) -> ();

    #[method(name = "get_NotLvUpTalkPids", args = 0)]
    pub fn get_not_lv_up_talk_pids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_NotLvUpTalkPids", args = 1)]
    pub fn set_not_lv_up_talk_pids(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_SummonColor", args = 0)]
    pub fn get_summon_color(self) -> crate::app::persondata::PersonData_Colors;

    #[method(name = "set_SummonColor", args = 1)]
    pub fn set_summon_color(self, value: crate::app::persondata::PersonData_Colors) -> ();

    #[method(name = "get_SummonRank", args = 0)]
    pub fn get_summon_rank(self) -> crate::app::persondata::PersonData_Ranks;

    #[method(name = "set_SummonRank", args = 1)]
    pub fn set_summon_rank(self, value: crate::app::persondata::PersonData_Ranks) -> ();

    #[method(name = "get_SummonGod", args = 0)]
    pub fn get_summon_god(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SummonGod", args = 1)]
    pub fn set_summon_god(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SummonRate", args = 0)]
    pub fn get_summon_rate(self) -> i32;

    #[method(name = "set_SummonRate", args = 1)]
    pub fn set_summon_rate(self, value: i32) -> ();

    #[method(name = "GetAutoGrowOffset", args = 0)]
    pub fn get_auto_grow_offset(self) -> i32;

    #[method(name = "GetCapabilityOffset", args = 0)]
    pub fn get_capability_offset(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "GetDressGender", args = 0)]
    pub fn get_dress_gender(self) -> crate::app::gender::Gender;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "OnCompletedEnd", args = 0)]
    pub fn on_completed_end(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "GetPrefixlessPid", args = 0)]
    pub fn get_prefixless_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "GetJob", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "get_MaskSkill", args = 0)]
    pub fn get_mask_skill(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "IsHero", args = 0)]
    pub fn is_hero(self) -> bool;

    #[method(name = "IsDerivedHero", args = 0)]
    pub fn is_derived_hero(self) -> bool;

    #[method(name = "IsDownload", args = 0)]
    pub fn is_download(self) -> bool;

    #[method(name = "IsUnknown", args = 0)]
    pub fn is_unknown(self) -> bool;

    #[method(name = "IsLevelUpTalkCombi", args = 1)]
    pub fn is_level_up_talk_combi(
        self,
        another_person_data: crate::app::persondata::PersonData,
    ) -> bool;

    #[method(name = "IsLevelUpTalkCombiInternal", args = 2)]
    pub fn is_level_up_talk_combi_internal(
        self,
        not_lv_up_talk_pid_array: ::unity2::Array<::unity2::Il2CppString>,
        target_pid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(
        stream: crate::app::stream_2::Stream_2,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(
        stream: crate::app::stream_2::Stream_2,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "TrySerialize", args = 2)]
    pub fn try_serialize(
        stream: crate::app::stream_2::Stream_2,
        person: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "TryDeserialize", args = 1)]
    pub fn try_deserialize(
        stream: crate::app::stream_2::Stream_2,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "IsVeyre", args = 1)]
    pub fn is_veyre(person: crate::app::persondata::PersonData) -> bool;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CommonSkills", args = 0)]
    pub fn get_common_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "set_CommonSkills", args = 1)]
    pub fn set_common_skills(self, value: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "get_NormalSkills", args = 0)]
    pub fn get_normal_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "set_NormalSkills", args = 1)]
    pub fn set_normal_skills(self, value: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "get_HardSkills", args = 0)]
    pub fn get_hard_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "set_HardSkills", args = 1)]
    pub fn set_hard_skills(self, value: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "get_LunaticSkills", args = 0)]
    pub fn get_lunatic_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "set_LunaticSkills", args = 1)]
    pub fn set_lunatic_skills(self, value: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "get_EngageSkill", args = 0)]
    pub fn get_engage_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_EngageSkill", args = 1)]
    pub fn set_engage_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_FaceData", args = 0)]
    pub fn get_face_data(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_FaceData", args = 1)]
    pub fn set_face_data(self, value: crate::app::persondata::PersonData) -> ();
}

#[cfg(feature = "app-persondata")]
impl PersonData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersonData),
                ::core::stringify!(new),
            )
        });
        <Self as IPersonDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PersonData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PersonData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PersonData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PersonData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PersonData_Flags {
    pub fn candidate_for_friend() -> Self {
        Self { value: 1 }
    }

    pub fn belong_name() -> Self {
        Self { value: 2 }
    }

    pub fn talent() -> Self {
        Self { value: 4 }
    }

    pub fn ignore_job_skill_remove() -> Self {
        Self { value: 8 }
    }

    pub fn dark_warp() -> Self {
        Self { value: 16 }
    }

    pub fn dress_reverse() -> Self {
        Self { value: 32 }
    }

    pub fn simple_ui() -> Self {
        Self { value: 64 }
    }

    pub fn derived_hero() -> Self {
        Self { value: 128 }
    }

    pub fn summon_warp() -> Self {
        Self { value: 256 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_Ranks.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PersonData_Ranks {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PersonData_Ranks {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PersonData.Ranks";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PersonData_Ranks {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PersonData_Ranks {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn rank1() -> Self {
        Self { value: 1 }
    }

    pub fn rank2() -> Self {
        Self { value: 2 }
    }

    pub fn rank3() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/persondata/PersonData_Country.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PersonData_Country {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PersonData_Country {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "PersonData.Country";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PersonData_Country {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PersonData_Country {
    pub fn free() -> Self {
        Self { value: 0 }
    }

    pub fn lithos() -> Self {
        Self { value: 1 }
    }

    pub fn filene() -> Self {
        Self { value: 2 }
    }

    pub fn brodia() -> Self {
        Self { value: 3 }
    }

    pub fn ircion() -> Self {
        Self { value: 4 }
    }

    pub fn solum() -> Self {
        Self { value: 5 }
    }

    pub fn gradlon() -> Self {
        Self { value: 6 }
    }
}
