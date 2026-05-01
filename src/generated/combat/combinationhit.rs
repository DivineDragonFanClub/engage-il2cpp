
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combinationhit/CombinationHit.md")))]
#[::unity2::class(namespace = "Combat", name = "CombinationHit")]
#[parent(crate::system::object::Object)]
pub struct CombinationHit {
    #[static_field]
    #[rename(name = "attackHashes")]
    pub attack_hashes: ::unity2::Array<i32>,
}

#[cfg(feature = "combat-combinationhit")]
#[::unity2::methods]
impl CombinationHit {
    #[method(name = "get_AttackHash", args = 0)]
    pub fn get_attack_hash(self) -> i32;

    #[method(name = "set_AttackHash", args = 1)]
    pub fn set_attack_hash(self, value: i32) -> ();

    #[method(name = "get_PlayFlags", args = 0)]
    pub fn get_play_flags(self) -> crate::combat::playflags::PlayFlags;

    #[method(name = "set_PlayFlags", args = 1)]
    pub fn set_play_flags(self, value: crate::combat::playflags::PlayFlags) -> ();

    #[method(name = "get_WaitTimeToAttack", args = 0)]
    pub fn get_wait_time_to_attack(self) -> f32;

    #[method(name = "set_WaitTimeToAttack", args = 1)]
    pub fn set_wait_time_to_attack(self, value: f32) -> ();

    #[method(name = "get_PlaybackRate", args = 0)]
    pub fn get_playback_rate(self) -> f32;

    #[method(name = "set_PlaybackRate", args = 1)]
    pub fn set_playback_rate(self, value: f32) -> ();

    #[method(name = "get_WorldHitTime", args = 0)]
    pub fn get_world_hit_time(self) -> f32;

    #[method(name = "set_WorldHitTime", args = 1)]
    pub fn set_world_hit_time(self, value: f32) -> ();

    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "set_CP", args = 1)]
    pub fn set_cp(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_AnimStartTime", args = 0)]
    pub fn get_anim_start_time(self) -> f32;

    #[method(name = "set_AnimStartTime", args = 1)]
    pub fn set_anim_start_time(self, value: f32) -> ();

    #[method(name = "get_AnimHitTime", args = 0)]
    pub fn get_anim_hit_time(self) -> f32;

    #[method(name = "set_AnimHitTime", args = 1)]
    pub fn set_anim_hit_time(self, value: f32) -> ();

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        self,
        side_attacker: crate::combat::character::Character,
        world_hit_time: f32,
        is_knockoff: bool,
    ) -> bool;

    #[method(name = "ListPossibleAttack", args = 1)]
    pub fn list_possible_attack(
        self,
        is_knockoff: bool,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::combat::prefetchedsignal::PrefetchedSignal,
    >;

    #[method(name = "SelectAttack", args = 3)]
    pub fn select_attack(
        self,
        list: crate::system::collections::generic::list_1::List_1<
            crate::combat::prefetchedsignal::PrefetchedSignal,
        >,
        magic: crate::combat::magic::Magic,
        remain_time: f32,
    ) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "combat-combinationhit")]
impl CombinationHit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombinationHit),
                ::core::stringify!(new),
            )
        });
        <Self as ICombinationHitMethods>::ctor(this);
        this
    }
}
