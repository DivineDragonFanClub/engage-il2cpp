
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/basecombatlocation/BaseCombatLocation.md")))]
#[::unity2::class(namespace = "Combat", name = "BaseCombatLocation")]
#[parent(crate::system::object::Object)]
pub struct BaseCombatLocation {
    #[static_field]
    #[rename(name = "m_PlayerDirHolder")]
    pub m_player_dir_holder: crate::app::gameparam::GameParam_Holder,
    #[static_field]
    #[rename(name = "m_EnemyDirHolder")]
    pub m_enemy_dir_holder: crate::app::gameparam::GameParam_Holder,
}

#[cfg(feature = "combat-basecombatlocation")]
#[::unity2::methods]
impl BaseCombatLocation {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_IsCalculated", args = 0)]
    pub fn get_is_calculated(self) -> bool;

    #[method(name = "set_IsCalculated", args = 1)]
    pub fn set_is_calculated(self, value: bool) -> ();

    #[method(name = "get_IsLocated", args = 0)]
    pub fn get_is_located(self) -> bool;

    #[method(name = "set_IsLocated", args = 1)]
    pub fn set_is_located(self, value: bool) -> ();

    #[method(name = "get_StandardFlyingHight", args = 0)]
    pub fn get_standard_flying_hight(self) -> f32;

    #[method(name = "get_ShootFlyingHight", args = 0)]
    pub fn get_shoot_flying_hight(self) -> f32;

    #[method(name = "get_HeightOverFlyingHight", args = 0)]
    pub fn get_height_over_flying_hight(self) -> f32;

    #[method(name = "get_TryCount", args = 0)]
    pub fn get_try_count(self) -> i32;

    #[method(name = "set_TryCount", args = 1)]
    pub fn set_try_count(self, value: i32) -> ();

    #[method(name = "get_Pattern", args = 0)]
    pub fn get_pattern(self) -> i32;

    #[method(name = "set_Pattern", args = 1)]
    pub fn set_pattern(self, value: i32) -> ();

    #[method(name = "get_UseOpenStyle", args = 0)]
    pub fn get_use_open_style(self) -> bool;

    #[method(name = "set_UseOpenStyle", args = 1)]
    pub fn set_use_open_style(self, value: bool) -> ();

    #[method(name = "CreateLocator", args = 1)]
    pub fn create_locator(
        record: crate::combat::combatrecord::CombatRecord,
    ) -> crate::combat::basecombatlocation::BaseCombatLocation;

    #[method(name = "Calculate", args = 1)]
    pub fn calculate(
        self,
        gs: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "Locate", args = 2)]
    pub fn locate(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
        locate_style: crate::combat::locationparams::LocationParams_LocateStyle,
    ) -> ();

    #[method(name = "LocateEmblem", args = 4)]
    pub fn locate_emblem(
        self,
        master_side: i32,
        master: crate::combat::character::Character,
        emblem: crate::combat::character::Character,
        locate_style: crate::combat::locationparams::LocationParams_LocateStyle,
    ) -> ();

    #[method(name = "get_Record", args = 0)]
    pub fn get_record(self) -> crate::combat::combatrecord::CombatRecord;

    #[method(name = "get_WorldPos", args = 0)]
    pub fn get_world_pos(self) -> ::unity2::Array<crate::combat::fxz::FXZ>;

    #[method(name = "set_WorldPos", args = 1)]
    pub fn set_world_pos(self, value: ::unity2::Array<crate::combat::fxz::FXZ>) -> ();

    #[method(name = "get_DamageCount", args = 0)]
    pub fn get_damage_count(self) -> ::unity2::Array<i32>;

    #[method(name = "set_DamageCount", args = 1)]
    pub fn set_damage_count(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "get_CharaTall", args = 0)]
    pub fn get_chara_tall(self) -> ::unity2::Array<f32>;

    #[method(name = "set_CharaTall", args = 1)]
    pub fn set_chara_tall(self, value: ::unity2::Array<f32>) -> ();

    #[method(name = "get_CharaBodySize", args = 0)]
    pub fn get_chara_body_size(self) -> ::unity2::Array<f32>;

    #[method(name = "set_CharaBodySize", args = 1)]
    pub fn set_chara_body_size(self, value: ::unity2::Array<f32>) -> ();

    #[method(name = "get_IsRiding", args = 0)]
    pub fn get_is_riding(self) -> ::unity2::Array<bool>;

    #[method(name = "set_IsRiding", args = 1)]
    pub fn set_is_riding(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_IsFlying", args = 0)]
    pub fn get_is_flying(self) -> ::unity2::Array<bool>;

    #[method(name = "set_IsFlying", args = 1)]
    pub fn set_is_flying(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_CanFly", args = 0)]
    pub fn get_can_fly(self) -> ::unity2::Array<bool>;

    #[method(name = "set_CanFly", args = 1)]
    pub fn set_can_fly(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_IsBigDragon", args = 0)]
    pub fn get_is_big_dragon(self) -> ::unity2::Array<bool>;

    #[method(name = "set_IsBigDragon", args = 1)]
    pub fn set_is_big_dragon(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_RoughPos", args = 0)]
    pub fn get_rough_pos(self) -> ::unity2::Array<crate::combat::fxz::FXZ>;

    #[method(name = "set_RoughPos", args = 1)]
    pub fn set_rough_pos(self, value: ::unity2::Array<crate::combat::fxz::FXZ>) -> ();

    #[method(name = "get_OutPos", args = 0)]
    pub fn get_out_pos(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_OutPos", args = 1)]
    pub fn set_out_pos(self, value: ::unity2::Array<crate::unity_engine::vector3::Vector3>) -> ();

    #[method(name = "get_OutDir", args = 0)]
    pub fn get_out_dir(self) -> ::unity2::Array<crate::combat::fxz::FXZ>;

    #[method(name = "set_OutDir", args = 1)]
    pub fn set_out_dir(self, value: ::unity2::Array<crate::combat::fxz::FXZ>) -> ();

    #[method(name = "get_SetFlying", args = 0)]
    pub fn get_set_flying(self) -> ::unity2::Array<bool>;

    #[method(name = "set_SetFlying", args = 1)]
    pub fn set_set_flying(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_SkipCheckOverWall", args = 0)]
    pub fn get_skip_check_over_wall(self) -> bool;

    #[method(name = "set_SkipCheckOverWall", args = 1)]
    pub fn set_skip_check_over_wall(self, value: bool) -> ();

    #[method(name = "get_StartDistance", args = 0)]
    pub fn get_start_distance(self) -> f32;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "BaseSetup", args = 1)]
    pub fn base_setup(
        self,
        gs: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(
        self,
        gs: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "get_RoughPosCount", args = 0)]
    pub fn get_rough_pos_count(self) -> i32;

    #[method(name = "SetRoughPos", args = 1)]
    pub fn set_rough_pos_2(self, try_count: i32) -> ();

    #[method(name = "get_PatternCount", args = 0)]
    pub fn get_pattern_count(self) -> i32;

    #[method(name = "SetBattlePatern", args = 1)]
    pub fn set_battle_patern(self, pattern: i32) -> ();

    #[method(name = "CalcLocation", args = 0)]
    pub fn calc_location(self) -> ();

    #[method(name = "get_SideAtk", args = 0)]
    pub fn get_side_atk(self) -> i32;

    #[method(name = "get_SideDmg", args = 0)]
    pub fn get_side_dmg(self) -> i32;

    #[method(name = "IsMoveOverWall", args = 0)]
    pub fn is_move_over_wall(self) -> bool;

    #[method(name = "IsMoveOverWall", args = 1)]
    pub fn is_move_over_wall_2(self, side: i32) -> bool;

    #[method(name = "IsOverWall", args = 5)]
    pub fn is_over_wall(self, s_x: i32, s_z: i32, g_x: i32, g_z: i32, is_fly: bool) -> bool;

    #[method(name = "IsWall", args = 2)]
    pub fn is_wall(self, side: i32, is_brawl: bool) -> bool;

    #[method(name = "IsWall", args = 4)]
    pub fn is_wall_2(self, x: i32, z: i32, is_brawl: bool, is_fly: bool) -> bool;

    #[method(name = "IsOverBorder", args = 1)]
    pub fn is_over_border(self, is_debug: bool) -> bool;

    #[method(name = "IsOverHeadBorder", args = 1)]
    pub fn is_over_head_border(self, is_debug: bool) -> bool;

    #[method(name = "IsOverHeadBorder", args = 2)]
    pub fn is_over_head_border_2(self, side: i32, is_debug: bool) -> bool;

    #[method(name = "IsRotateTooMuch", args = 0)]
    pub fn is_rotate_too_much(self) -> bool;

    #[method(name = "IsStandable", args = 1)]
    pub fn is_standable(self, is_debug: bool) -> bool;

    #[method(name = "IsStandable", args = 2)]
    pub fn is_standable_2(self, side: i32, is_debug: bool) -> bool;

    #[method(name = "IsStandable", args = 3)]
    pub fn is_standable_3(self, side: i32, size: f32, is_debug: bool) -> bool;

    #[method(name = "IsStandable", args = 4)]
    pub fn is_standable_4(
        self,
        center: crate::unity_engine::vector3::Vector3,
        size: f32,
        mask: i32,
        is_debug: bool,
    ) -> bool;

    #[method(name = "IsOutOfMap", args = 0)]
    pub fn is_out_of_map(self) -> bool;

    #[method(name = "AboidStep", args = 0)]
    pub fn aboid_step(self) -> ();

    #[method(name = "AboidStep", args = 2)]
    pub fn aboid_step_2(self, side: i32, dist: f32) -> ();

    #[method(name = "GetHeightForFly", args = 3)]
    pub fn get_height_for_fly(self, pos: crate::combat::fxz::FXZ, fly: f32, is_debug: bool) -> f32;

    #[method(name = "GetDist2CliffOrWall", args = 4)]
    pub fn get_dist2_cliff_or_wall(
        self,
        side: i32,
        dir: crate::combat::fxz::FXZ,
        check_dist: f32,
        is_debug: bool,
    ) -> f32;

    #[method(name = "GetDist2Cliff", args = 4)]
    pub fn get_dist2_cliff(
        self,
        side: i32,
        dir: crate::combat::fxz::FXZ,
        check_dist: f32,
        is_debug: bool,
    ) -> f32;

    #[method(name = "GetDist2Wall", args = 4)]
    pub fn get_dist2_wall(
        self,
        side: i32,
        dir: crate::combat::fxz::FXZ,
        check_dist: f32,
        is_debug: bool,
    ) -> f32;

    #[method(name = "IsShootable", args = 1)]
    pub fn is_shootable(self, is_debug: bool) -> bool;

    #[method(name = "CheckCameraSpace", args = 4)]
    pub fn check_camera_space(
        self,
        side: i32,
        forward_dist: f32,
        side_dist: f32,
        is_debug: bool,
    ) -> bool;

    #[method(name = "LocateForSolo", args = 2)]
    pub fn locate_for_solo(self, side: i32, try_count: i32) -> bool;

    #[method(name = "LocateToOpenSpace", args = 1)]
    pub fn locate_to_open_space(self, is_debug: bool) -> bool;

    #[method(name = "RelocateToSpace", args = 1)]
    pub fn relocate_to_space(self, is_start: bool) -> ();

    #[method(name = "LocateForWin", args = 1)]
    pub fn locate_for_win(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "LocateSoloForWin", args = 4)]
    pub fn locate_solo_for_win(
        self,
        side: i32,
        check_en_pos: bool,
        forward: crate::combat::fxz::FXZ,
        check_radius: f32,
    ) -> i32;

    #[method(name = "LocateForSkip", args = 1)]
    pub fn locate_for_skip(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "LocateForEndTraining", args = 1)]
    pub fn locate_for_end_training(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "LocateForChainAttack", args = 4)]
    pub fn locate_for_chain_attack(
        self,
        chain: crate::combat::character::Character,
        chars: ::unity2::Array<crate::combat::character::Character>,
        attack_hash: i32,
        chain_attack_time: f32,
    ) -> ();

    #[method(name = "RelocateAfterChainGuard", args = 2)]
    pub fn relocate_after_chain_guard(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
        guard: crate::combat::character::Character,
    ) -> ();

    #[method(name = "LocateEmblemAfterCombat", args = 1)]
    pub fn locate_emblem_after_combat(self, emblem: crate::combat::character::Character) -> ();

    #[method(name = "LoadCurrentPos", args = 1)]
    pub fn load_current_pos(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "GetSoloCharacterForward", args = 1)]
    pub fn get_solo_character_forward(is_enemy: bool) -> crate::combat::fxz::FXZ;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "combat-basecombatlocation")]
impl BaseCombatLocation {
    pub fn new(record: crate::combat::combatrecord::CombatRecord) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseCombatLocation),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseCombatLocationMethods>::ctor(this, record);
        this
    }
}
