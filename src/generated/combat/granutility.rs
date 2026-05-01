
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/granutility/GranUtility.md")))]
#[::unity2::class(namespace = "Combat", name = "GranUtility")]
#[parent(crate::system::object::Object)]
pub struct GranUtility {}

#[cfg(feature = "combat-granutility")]
#[::unity2::methods]
impl GranUtility {
    #[method(name = "CalcAttackStance2", args = 3)]
    pub fn calc_attack_stance2(
        ghr: crate::combat::character::Character,
        distance: f32,
        allrange: bool,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackStanceNN", args = 2)]
    pub fn calc_attack_stance_nn(
        ghr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackStanceFN", args = 2)]
    pub fn calc_attack_stance_fn(
        ghr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackStanceFF", args = 1)]
    pub fn calc_attack_stance_ff(ghr: crate::combat::character::Character)
        -> crate::combat::tr::TR;

    #[method(name = "CalcAttackStanceNF", args = 1)]
    pub fn calc_attack_stance_nf(ghr: crate::combat::character::Character)
        -> crate::combat::tr::TR;

    #[method(name = "CalcShootStance", args = 1)]
    pub fn calc_shoot_stance(grandew: crate::combat::character::Character)
        -> crate::combat::tr::TR;

    #[method(name = "CalcWaitPosition", args = 3)]
    pub fn calc_wait_position(
        grandew: crate::combat::character::Character,
        distance: f32,
        degree: f32,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcWaitPosition", args = 5)]
    pub fn calc_wait_position_2(
        master: crate::combat::character::Character,
        grandew: crate::combat::character::Character,
        enemy: crate::combat::character::Character,
        distance: f32,
        degree: f32,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcDamageStance", args = 1)]
    pub fn calc_damage_stance(
        grandew: crate::combat::character::Character,
    ) -> crate::combat::tr::TR;

    #[method(name = "Calc見切りStance", args = 1)]
    pub fn calc____stance(grandew: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "CalcPoint", args = 4)]
    pub fn calc_point(
        target: crate::combat::fxz::FXZ,
        dir_to_target: crate::combat::fxz::FXZ,
        rad: f32,
        distance: f32,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcNearAttackStance", args = 2)]
    pub fn calc_near_attack_stance(
        grandew: crate::combat::character::Character,
        distance: f32,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcNearDamageStance", args = 2)]
    pub fn calc_near_damage_stance(
        grandew: crate::combat::character::Character,
        distance: f32,
    ) -> crate::combat::tr::TR;

    #[method(name = "CalcBehindAttackLine", args = 4)]
    pub fn calc_behind_attack_line(
        plpos: crate::combat::fxz::FXZ,
        enpos: crate::combat::fxz::FXZ,
        ratio: f32,
        distance: f32,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "CalcBehindAttackLine2", args = 5)]
    pub fn calc_behind_attack_line2(
        plpos: crate::combat::fxz::FXZ,
        enpos: crate::combat::fxz::FXZ,
        ratio: f32,
        angle: f32,
        distance: f32,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "CalcBehindAttackLine3", args = 4)]
    pub fn calc_behind_attack_line3(
        grandew: crate::combat::character::Character,
        ratio: f32,
        angle: f32,
        distance: f32,
    ) -> crate::combat::fxz::FXZ;

    #[method(name = "CalcBehindAttackLine4", args = 6)]
    pub fn calc_behind_attack_line4(
        grandew: crate::combat::character::Character,
        plpos: crate::combat::fxz::FXZ,
        enpos: crate::combat::fxz::FXZ,
        ratio: f32,
        angle: f32,
        distance: f32,
    ) -> crate::combat::fxz::FXZ;
}
