
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/goddata/GodData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodData_Flags {
    pub fn no_add_exp() -> Self {
        Self { value: 1 }
    }

    pub fn enable_ring_list() -> Self {
        Self { value: 2 }
    }

    pub fn unit_icon_darkness() -> Self {
        Self { value: 4 }
    }

    pub fn gauge_darkness() -> Self {
        Self { value: 8 }
    }

    pub fn only_engage_weapon() -> Self {
        Self { value: 16 }
    }

    pub fn armlet() -> Self {
        Self { value: 32 }
    }

    pub fn hero() -> Self {
        Self { value: -2147483648 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/goddata/GodData_RelianceLevel.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodData_RelianceLevel {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodData_RelianceLevel {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodData.RelianceLevel";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodData_RelianceLevel {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodData_RelianceLevel {
    pub fn d() -> Self {
        Self { value: 0 }
    }

    pub fn c() -> Self {
        Self { value: 1 }
    }

    pub fn b() -> Self {
        Self { value: 2 }
    }

    pub fn a() -> Self {
        Self { value: 3 }
    }

    pub fn s() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/goddata/GodData.md")))]
#[::unity2::class(namespace = "App", name = "GodData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: goddata :: GodData >)]
pub struct GodData {
    #[rename(name = "m_EngageHauntUnit")]
    pub m_engage_haunt_unit: crate::app::unit::Unit,
    #[static_field]
    #[rename(name = "s_LinkDics")]
    pub s_link_dics: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::goddata::GodData,
    >,
}

#[cfg(feature = "app-goddata")]
#[::unity2::methods]
impl GodData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Gid", args = 0)]
    pub fn get_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Gid", args = 1)]
    pub fn set_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Mid", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Mid", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Nickname", args = 0)]
    pub fn get_nickname(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Nickname", args = 1)]
    pub fn set_nickname(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SoundID", args = 0)]
    pub fn get_sound_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SoundID", args = 1)]
    pub fn set_sound_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AssetID", args = 0)]
    pub fn get_asset_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AssetID", args = 1)]
    pub fn set_asset_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FaceIconName", args = 0)]
    pub fn get_face_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FaceIconName", args = 1)]
    pub fn set_face_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FaceIconNameDarkness", args = 0)]
    pub fn get_face_icon_name_darkness(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FaceIconNameDarkness", args = 1)]
    pub fn set_face_icon_name_darkness(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Ringname", args = 0)]
    pub fn get_ringname(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Ringname", args = 1)]
    pub fn set_ringname(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Ringhelp", args = 0)]
    pub fn get_ringhelp(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Ringhelp", args = 1)]
    pub fn set_ringhelp(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_UnitIconID", args = 0)]
    pub fn get_unit_icon_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitIconID", args = 1)]
    pub fn set_unit_icon_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Change", args = 0)]
    pub fn get_change(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Change", args = 1)]
    pub fn set_change(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Link", args = 0)]
    pub fn get_link(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Link", args = 1)]
    pub fn set_link(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EngageHaunt", args = 0)]
    pub fn get_engage_haunt(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngageHaunt", args = 1)]
    pub fn set_engage_haunt(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "set_ForceType", args = 1)]
    pub fn set_force_type(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_Female", args = 0)]
    pub fn get_female(self) -> i8;

    #[method(name = "set_Female", args = 1)]
    pub fn set_female(self, value: i8) -> ();

    #[method(name = "get_GoodWeapon", args = 0)]
    pub fn get_good_weapon(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "set_GoodWeapon", args = 1)]
    pub fn set_good_weapon(self, value: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "get_Sort", args = 0)]
    pub fn get_sort(self) -> i16;

    #[method(name = "set_Sort", args = 1)]
    pub fn set_sort(self, value: i16) -> ();

    #[method(name = "get_EngageCount", args = 0)]
    pub fn get_engage_count(self) -> u8;

    #[method(name = "set_EngageCount", args = 1)]
    pub fn set_engage_count(self, value: u8) -> ();

    #[method(name = "get_EngageAttack", args = 0)]
    pub fn get_engage_attack(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngageAttack", args = 1)]
    pub fn set_engage_attack(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EngageAttackRampage", args = 0)]
    pub fn get_engage_attack_rampage(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngageAttackRampage", args = 1)]
    pub fn set_engage_attack_rampage(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EngageAttackLink", args = 0)]
    pub fn get_engage_attack_link(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngageAttackLink", args = 1)]
    pub fn set_engage_attack_link(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LinkGid", args = 0)]
    pub fn get_link_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LinkGid", args = 1)]
    pub fn set_link_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Gbid", args = 0)]
    pub fn get_gbid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Gbid", args = 1)]
    pub fn set_gbid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GrowTable", args = 0)]
    pub fn get_grow_table(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GrowTable", args = 1)]
    pub fn set_grow_table(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LevelCap", args = 0)]
    pub fn get_level_cap(self) -> u8;

    #[method(name = "set_LevelCap", args = 1)]
    pub fn set_level_cap(self, value: u8) -> ();

    #[method(name = "get_UnlockLevelCapVarName", args = 0)]
    pub fn get_unlock_level_cap_var_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnlockLevelCapVarName", args = 1)]
    pub fn set_unlock_level_cap_var_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EngraveWord", args = 0)]
    pub fn get_engrave_word(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EngraveWord", args = 1)]
    pub fn set_engrave_word(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EngravePower", args = 0)]
    pub fn get_engrave_power(self) -> i8;

    #[method(name = "set_EngravePower", args = 1)]
    pub fn set_engrave_power(self, value: i8) -> ();

    #[method(name = "get_EngraveWeight", args = 0)]
    pub fn get_engrave_weight(self) -> i8;

    #[method(name = "set_EngraveWeight", args = 1)]
    pub fn set_engrave_weight(self, value: i8) -> ();

    #[method(name = "get_EngraveHit", args = 0)]
    pub fn get_engrave_hit(self) -> i8;

    #[method(name = "set_EngraveHit", args = 1)]
    pub fn set_engrave_hit(self, value: i8) -> ();

    #[method(name = "get_EngraveCritical", args = 0)]
    pub fn get_engrave_critical(self) -> i8;

    #[method(name = "set_EngraveCritical", args = 1)]
    pub fn set_engrave_critical(self, value: i8) -> ();

    #[method(name = "get_EngraveAvoid", args = 0)]
    pub fn get_engrave_avoid(self) -> i8;

    #[method(name = "set_EngraveAvoid", args = 1)]
    pub fn set_engrave_avoid(self, value: i8) -> ();

    #[method(name = "get_EngraveSecure", args = 0)]
    pub fn get_engrave_secure(self) -> i8;

    #[method(name = "set_EngraveSecure", args = 1)]
    pub fn set_engrave_secure(self, value: i8) -> ();

    #[method(name = "get_SynchroEnhance", args = 0)]
    pub fn get_synchro_enhance(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "set_SynchroEnhance", args = 1)]
    pub fn set_synchro_enhance(self, value: crate::app::capabilitysbyte::CapabilitySbyte) -> ();

    #[method(name = "get_MainData", args = 0)]
    pub fn get_main_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_MainData", args = 1)]
    pub fn set_main_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "get_ChangeData", args = 0)]
    pub fn get_change_data(self) -> ::unity2::Array<crate::app::goddata::GodData>;

    #[method(name = "set_ChangeData", args = 1)]
    pub fn set_change_data(self, value: ::unity2::Array<crate::app::goddata::GodData>) -> ();

    #[method(name = "get_ChangeIndex", args = 0)]
    pub fn get_change_index(self) -> i32;

    #[method(name = "set_ChangeIndex", args = 1)]
    pub fn set_change_index(self, value: i32) -> ();

    #[method(name = "get_AsciiName", args = 0)]
    pub fn get_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AsciiName", args = 1)]
    pub fn set_ascii_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::goddata::GodData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::goddata::GodData_FlagField) -> ();

    #[method(name = "get_NetRankingIndex", args = 0)]
    pub fn get_net_ranking_index(self) -> u8;

    #[method(name = "set_NetRankingIndex", args = 1)]
    pub fn set_net_ranking_index(self, value: u8) -> ();

    #[method(name = "get_HeroFaceIconName", args = 0)]
    pub fn get_hero_face_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HeroFaceIconName", args = 1)]
    pub fn set_hero_face_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HeroineFaceIconName", args = 0)]
    pub fn get_heroine_face_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HeroineFaceIconName", args = 1)]
    pub fn set_heroine_face_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AIEngageAttackType", args = 0)]
    pub fn get_ai_engage_attack_type(self) -> crate::app::goddata::GodData_AIEngageAttackTypes;

    #[method(name = "set_AIEngageAttackType", args = 1)]
    pub fn set_ai_engage_attack_type(
        self,
        value: crate::app::goddata::GodData_AIEngageAttackTypes,
    ) -> ();

    #[method(name = "GetLinkGodData", args = 0)]
    pub fn get_link_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "GetEngageHauntUnit", args = 0)]
    pub fn get_engage_haunt_unit(self) -> crate::app::unit::Unit;

    #[method(name = "TryGetLink", args = 1)]
    pub fn try_get_link(person: crate::app::persondata::PersonData)
        -> crate::app::goddata::GodData;

    #[method(name = "CalcChangeData", args = 0)]
    pub fn calc_change_data(self) -> ::unity2::Array<crate::app::goddata::GodData>;

    #[method(name = "CalcChangeIndex", args = 0)]
    pub fn calc_change_index(self) -> i32;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetGender", args = 0)]
    pub fn get_gender(self) -> crate::app::gender::Gender;

    #[method(name = "GetPrefixlessGid", args = 0)]
    pub fn get_prefixless_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "IsHero", args = 0)]
    pub fn is_hero(self) -> bool;

    #[method(name = "GetEngageZonePrefabPath", args = 1)]
    pub fn get_engage_zone_prefab_path(gid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetChangeDataCount", args = 0)]
    pub fn get_change_data_count(self) -> i32;

    #[method(name = "GetChangeData", args = 1)]
    pub fn get_change_data_2(self, index: i32) -> crate::app::goddata::GodData;

    #[method(name = "GetRandomChangeData", args = 1)]
    pub fn get_random_change_data(
        self,
        random: crate::app::random_2::Random_2,
    ) -> crate::app::goddata::GodData;

    #[method(name = "GetNextChangeData", args = 0)]
    pub fn get_next_change_data(self) -> crate::app::goddata::GodData;

    #[method(name = "GetInfoData", args = 0)]
    pub fn get_info_data(self) -> crate::app::goddata::GodData;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-goddata")]
impl GodData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodData),
                ::core::stringify!(new),
            )
        });
        <Self as IGodDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/goddata/GodData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "GodData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: goddata :: GodData_Flags >)]
pub struct GodData_FlagField {}

#[cfg(feature = "app-goddata")]
#[::unity2::methods]
impl GodData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::goddata::GodData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::goddata::GodData_Flags) -> i32;
}

#[cfg(feature = "app-goddata")]
impl GodData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IGodData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::goddata::GodData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGodData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/goddata/GodData_AIEngageAttackTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GodData_AIEngageAttackTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GodData_AIEngageAttackTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GodData.AIEngageAttackTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GodData_AIEngageAttackTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GodData_AIEngageAttackTypes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn attack() -> Self {
        Self { value: 1 }
    }

    pub fn attack_pierce() -> Self {
        Self { value: 2 }
    }

    pub fn attack_charge() -> Self {
        Self { value: 3 }
    }

    pub fn heal() -> Self {
        Self { value: 4 }
    }

    pub fn dance() -> Self {
        Self { value: 5 }
    }

    pub fn bless() -> Self {
        Self { value: 6 }
    }

    pub fn attack_wait() -> Self {
        Self { value: 7 }
    }

    pub fn overlap() -> Self {
        Self { value: 8 }
    }

    pub fn summon() -> Self {
        Self { value: 9 }
    }
}
