
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/decorator/Decorator.md")))]
#[::unity2::class(namespace = "Combat", name = "Decorator")]
#[parent(crate::system::object::Object)]
pub struct Decorator {
    #[rename(name = "m_Side")]
    pub m_side: i32,
}

#[cfg(feature = "combat-decorator")]
#[::unity2::methods]
impl Decorator {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_m_Phase", args = 0)]
    pub fn get_m_phase(self) -> crate::combat::phase::Phase;

    #[method(name = "set_m_Phase", args = 1)]
    pub fn set_m_phase(self, value: crate::combat::phase::Phase) -> ();

    #[method(name = "get_m_Chrs", args = 0)]
    pub fn get_m_chrs(self) -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "get_m_Master", args = 0)]
    pub fn get_m_master(self) -> crate::combat::character::Character;

    #[method(name = "get_m_Enemy", args = 0)]
    pub fn get_m_enemy(self) -> crate::combat::character::Character;

    #[method(name = "get_m_Grandew", args = 0)]
    pub fn get_m_grandew(self) -> crate::combat::character::Character;

    #[method(name = "get_m_EnemyGrandew", args = 0)]
    pub fn get_m_enemy_grandew(self) -> crate::combat::character::Character;

    #[method(name = "UseOnArrivalTimePredicted", args = 0)]
    pub fn use_on_arrival_time_predicted(self) -> ();

    #[method(name = "UseOnHitTimePredicted", args = 0)]
    pub fn use_on_hit_time_predicted(self) -> ();

    #[method(name = "UseOnEnemyDamage", args = 0)]
    pub fn use_on_enemy_damage(self) -> ();

    #[method(name = "UseOnBackwardCancel", args = 1)]
    pub fn use_on_backward_cancel(self, chr: crate::combat::character::Character) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnEnterAttack", args = 0)]
    pub fn on_enter_attack(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "OnArrivalTimePredicted_", args = 1)]
    pub fn on_arrival_time_predicted(self, world_arrival_time: f32) -> ();

    #[method(name = "OnHitTimePredicted_", args = 2)]
    pub fn on_hit_time_predicted(
        self,
        world_hit_time: f32,
        weapon_style: crate::combat::weaponstyle::WeaponStyle,
    ) -> ();

    #[method(name = "OnShoot", args = 1)]
    pub fn on_shoot(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "OnMissPassed", args = 1)]
    pub fn on_miss_passed(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "OnHitPassed", args = 1)]
    pub fn on_hit_passed(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "OnEnemyDamage_", args = 1)]
    pub fn on_enemy_damage(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "OnBackwardCancel_", args = 0)]
    pub fn on_backward_cancel(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-decorator")]
impl Decorator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Decorator),
                ::core::stringify!(new),
            )
        });
        <Self as IDecoratorMethods>::ctor(this);
        this
    }
}
