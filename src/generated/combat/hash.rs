
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/hash/Hash.md")))]
#[::unity2::class(namespace = "Combat", name = "Hash")]
#[parent(crate::system::object::Object)]
pub struct Hash {
    #[static_field]
    #[rename(name = "Attack1")]
    pub attack1: i32,
    #[static_field]
    #[rename(name = "Attack2")]
    pub attack2: i32,
    #[static_field]
    #[rename(name = "Attack3")]
    pub attack3: i32,
    #[static_field]
    #[rename(name = "Attack4")]
    pub attack4: i32,
    #[static_field]
    #[rename(name = "Attack5")]
    pub attack5: i32,
    #[static_field]
    #[rename(name = "AttackC")]
    pub attack_c: i32,
    #[static_field]
    #[rename(name = "AttackT")]
    pub attack_t: i32,
    #[static_field]
    #[rename(name = "DamageHigh")]
    pub damage_high: i32,
    #[static_field]
    #[rename(name = "DamageMidB")]
    pub damage_mid_b: i32,
    #[static_field]
    #[rename(name = "DamageMidDU")]
    pub damage_mid_du: i32,
    #[static_field]
    #[rename(name = "DamageMidUD")]
    pub damage_mid_ud: i32,
    #[static_field]
    #[rename(name = "DieB")]
    pub die_b: i32,
    #[static_field]
    #[rename(name = "DieL")]
    pub die_l: i32,
    #[static_field]
    #[rename(name = "DieR")]
    pub die_r: i32,
    #[static_field]
    #[rename(name = "Dive")]
    pub dive: i32,
    #[static_field]
    #[rename(name = "Engage1")]
    pub engage1: i32,
    #[static_field]
    #[rename(name = "Engage2")]
    pub engage2: i32,
    #[static_field]
    #[rename(name = "Engage3")]
    pub engage3: i32,
    #[static_field]
    #[rename(name = "EvasionB")]
    pub evasion_b: i32,
    #[static_field]
    #[rename(name = "EvasionL")]
    pub evasion_l: i32,
    #[static_field]
    #[rename(name = "EvasionR")]
    pub evasion_r: i32,
    #[static_field]
    #[rename(name = "Guard")]
    pub guard: i32,
    #[static_field]
    #[rename(name = "HoveringLoop")]
    pub hovering_loop: i32,
    #[static_field]
    #[rename(name = "Idle")]
    pub idle: i32,
    #[static_field]
    #[rename(name = "ParryL")]
    pub parry_l: i32,
    #[static_field]
    #[rename(name = "ParryR")]
    pub parry_r: i32,
    #[static_field]
    #[rename(name = "Ready")]
    pub ready: i32,
    #[static_field]
    #[rename(name = "RelaxLoop")]
    pub relax_loop: i32,
    #[static_field]
    #[rename(name = "Repelled")]
    pub repelled: i32,
    #[static_field]
    #[rename(name = "RunLoop")]
    pub run_loop: i32,
    #[static_field]
    #[rename(name = "RunStart")]
    pub run_start: i32,
    #[static_field]
    #[rename(name = "Special1")]
    pub special1: i32,
    #[static_field]
    #[rename(name = "Start")]
    pub start: i32,
    #[static_field]
    #[rename(name = "Win")]
    pub win: i32,
    #[static_field]
    #[rename(name = "WinLoop")]
    pub win_loop: i32,
    #[static_field]
    #[rename(name = "AttackAll")]
    pub attack_all: i32,
    #[static_field]
    #[rename(name = "DamageMidAll")]
    pub damage_mid_all: i32,
    #[static_field]
    #[rename(name = "DieAll")]
    pub die_all: i32,
    #[static_field]
    #[rename(name = "EvasionAll")]
    pub evasion_all: i32,
    #[static_field]
    #[rename(name = "ParryAll")]
    pub parry_all: i32,
    #[static_field]
    #[rename(name = "PlaybackRate")]
    pub playback_rate: i32,
    #[static_field]
    #[rename(name = "IsRunning")]
    pub is_running: i32,
    #[static_field]
    #[rename(name = "Dying")]
    pub dying: i32,
    #[static_field]
    #[rename(name = "FacialNormal")]
    pub facial_normal: i32,
    #[static_field]
    #[rename(name = "FacialPain")]
    pub facial_pain: i32,
    #[static_field]
    #[rename(name = "FacialDie")]
    pub facial_die: i32,
}

#[cfg(feature = "combat-hash")]
#[::unity2::methods]
impl Hash {
    #[method(name = "ToHash", args = 1)]
    pub fn to_hash(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "ToName", args = 1)]
    pub fn to_name(hash: i32) -> ::unity2::Il2CppString;

    #[method(name = "get_HashArray", args = 0)]
    pub fn get_hash_array() -> ::unity2::Array<i32>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
