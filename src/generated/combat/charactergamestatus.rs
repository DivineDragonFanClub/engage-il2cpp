
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactergamestatus/CharacterGameStatus.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterGameStatus")]
#[parent(crate::system::object::Object)]
pub struct CharacterGameStatus {
    #[rename(name = "Appearance")]
    pub appearance: crate::combat::characterappearance::CharacterAppearance,
    #[rename(name = "m_Side")]
    pub m_side: i32,
    #[rename(name = "m_bStun")]
    pub m_b_stun: bool,
}

#[cfg(feature = "combat-charactergamestatus")]
#[::unity2::methods]
impl CharacterGameStatus {
    #[method(name = "get_DebuggerDisplay", args = 0)]
    pub fn get_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(a: crate::combat::charactergamestatus::CharacterGameStatus) -> bool;

    #[method(name = "get_IsEmpty", args = 0)]
    pub fn get_is_empty(self) -> bool;

    #[method(name = "get_EmblemIdentifier", args = 0)]
    pub fn get_emblem_identifier(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EmblemIdentifier", args = 1)]
    pub fn set_emblem_identifier(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Stun", args = 0)]
    pub fn stun(self) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: crate::app::persondata::PersonData) -> ();

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "set_Job", args = 1)]
    pub fn set_job(self, value: crate::app::jobdata::JobData) -> ();

    #[method(name = "get_Force", args = 0)]
    pub fn get_force(self) -> crate::app::force::Force;

    #[method(name = "set_Force", args = 1)]
    pub fn set_force(self, value: crate::app::force::Force) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MaxHP", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "set_MaxHP", args = 1)]
    pub fn set_max_hp(self, value: i32) -> ();

    #[method(name = "get_HP", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "set_HP", args = 1)]
    pub fn set_hp(self, value: i32) -> ();

    #[method(name = "get_MaxStun", args = 0)]
    pub fn get_max_stun(self) -> i32;

    #[method(name = "set_MaxStun", args = 1)]
    pub fn set_max_stun(self, value: i32) -> ();

    #[method(name = "get_StunValue", args = 0)]
    pub fn get_stun_value(self) -> i32;

    #[method(name = "set_StunValue", args = 1)]
    pub fn set_stun_value(self, value: i32) -> ();

    #[method(name = "get_IsStun", args = 0)]
    pub fn get_is_stun(self) -> bool;

    #[method(name = "get_EngageCount", args = 0)]
    pub fn get_engage_count(self) -> i32;

    #[method(name = "set_EngageCount", args = 1)]
    pub fn set_engage_count(self, value: i32) -> ();

    #[method(name = "get_MapX", args = 0)]
    pub fn get_map_x(self) -> i32;

    #[method(name = "set_MapX", args = 1)]
    pub fn set_map_x(self, value: i32) -> ();

    #[method(name = "get_MapY", args = 0)]
    pub fn get_map_y(self) -> i32;

    #[method(name = "set_MapY", args = 1)]
    pub fn set_map_y(self, value: i32) -> ();

    #[method(name = "get_BattleX", args = 0)]
    pub fn get_battle_x(self) -> i32;

    #[method(name = "set_BattleX", args = 1)]
    pub fn set_battle_x(self, value: i32) -> ();

    #[method(name = "get_BattleY", args = 0)]
    pub fn get_battle_y(self) -> i32;

    #[method(name = "set_BattleY", args = 1)]
    pub fn set_battle_y(self, value: i32) -> ();

    #[method(name = "get_Weapon", args = 0)]
    pub fn get_weapon(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_Weapon", args = 1)]
    pub fn set_weapon(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_WeaponStyle", args = 0)]
    pub fn get_weapon_style(self) -> crate::combat::weaponstyle::WeaponStyle;

    #[method(name = "get_EngageStyle", args = 0)]
    pub fn get_engage_style(self) -> crate::combat::engagestyle::EngageStyle;

    #[method(name = "set_EngageStyle", args = 1)]
    pub fn set_engage_style(self, value: crate::combat::engagestyle::EngageStyle) -> ();

    #[method(name = "SetEngageStyleAsEngAttackPairedGrandew", args = 0)]
    pub fn set_engage_style_as_eng_attack_paired_grandew(self) -> ();

    #[method(name = "get_IsEngaging", args = 0)]
    pub fn get_is_engaging(self) -> bool;

    #[method(name = "get_IsGrandew", args = 0)]
    pub fn get_is_grandew(self) -> bool;

    #[method(name = "get_IsDying", args = 0)]
    pub fn get_is_dying(self) -> bool;

    #[method(name = "get_IsDead", args = 0)]
    pub fn get_is_dead(self) -> bool;

    #[method(name = "get_IsHPMax", args = 0)]
    pub fn get_is_hp_max(self) -> bool;

    #[method(name = "get_StarRushComboCount789", args = 0)]
    pub fn get_star_rush_combo_count789(self) -> i32;

    #[method(name = "set_StarRushComboCount789", args = 1)]
    pub fn set_star_rush_combo_count789(self, value: i32) -> ();

    #[method(name = "get_後キャン発動位置", args = 0)]
    pub fn get_________(self) -> f32;

    #[method(name = "set_後キャン発動位置", args = 1)]
    pub fn set_________(self, value: f32) -> ();

    #[method(name = "get_ブレイク時後キャン発動位置", args = 0)]
    pub fn get______________(self) -> f32;

    #[method(name = "set_ブレイク時後キャン発動位置", args = 1)]
    pub fn set______________(self, value: f32) -> ();

    #[method(name = "get_重い動作速度", args = 0)]
    pub fn get_______(self) -> f32;

    #[method(name = "set_重い動作速度", args = 1)]
    pub fn set_______(self, value: f32) -> ();

    #[method(name = "get_素早い動作速度", args = 0)]
    pub fn get________(self) -> f32;

    #[method(name = "set_素早い動作速度", args = 1)]
    pub fn set________(self, value: f32) -> ();

    #[method(name = "get_WorldPos", args = 0)]
    pub fn get_world_pos(self) -> crate::combat::fxz::FXZ;

    #[method(name = "set_WorldPos", args = 1)]
    pub fn set_world_pos(self, value: crate::combat::fxz::FXZ) -> ();

    #[method(name = "get_RodUseType", args = 0)]
    pub fn get_rod_use_type(self) -> crate::app::itemdata::ItemData_UseTypes;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Import", args = 3)]
    pub fn import(
        self,
        unit: crate::app::unit::Unit,
        is_hp_max: bool,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "Import", args = 3)]
    pub fn import_2(
        self,
        unit: crate::app::unit::Unit,
        weapon: crate::app::unititem::UnitItem,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "Import", args = 2)]
    pub fn import_3(
        self,
        god: crate::app::godunit::GodUnit,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "Import", args = 4)]
    pub fn import_4(
        self,
        side: i32,
        calc: crate::app::battlecalculator::BattleCalculator,
        side_type: crate::app::battleside::BattleSide_Type,
        map_distance: i32,
    ) -> ();

    #[method(name = "Import", args = 7)]
    pub fn import_5(
        self,
        side: i32,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        iid: ::unity2::Il2CppString,
        x: i32,
        z: i32,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "ImportEmpty", args = 0)]
    pub fn import_empty(self) -> ();

    #[method(name = "ImportImpl", args = 3)]
    pub fn import_impl(
        self,
        weapon: crate::app::unititem::UnitItem,
        map_distance: i32,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "UpdatePos", args = 0)]
    pub fn update_pos(self) -> ();

    #[method(name = "CalcEngageStyle", args = 0)]
    pub fn calc_engage_style(self) -> crate::combat::engagestyle::EngageStyle;

    #[method(name = "ImportGrandew", args = 1)]
    pub fn import_grandew(
        self,
        master: crate::combat::charactergamestatus::CharacterGameStatus,
    ) -> ();

    #[method(name = "InitForViewer", args = 3)]
    pub fn init_for_viewer(
        self,
        side: i32,
        ap: crate::combat::characterappearance::CharacterAppearance,
        engage_style: crate::combat::engagestyle::EngageStyle,
    ) -> ();

    #[method(name = "SyncParams", args = 1)]
    pub fn sync_params(self, phase: crate::combat::phase::Phase) -> ();

    #[method(name = "RemoveWeapons", args = 0)]
    pub fn remove_weapons(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactergamestatus")]
impl CharacterGameStatus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterGameStatus),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterGameStatusMethods>::ctor(this);
        this
    }
}
