
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactereffect/CharacterEffect.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterEffect")]
#[parent(crate::system::object::Object)]
pub struct CharacterEffect {
    #[rename(name = "CP")]
    pub cp: crate::combat::character::Character,
}

#[cfg(feature = "combat-charactereffect")]
#[::unity2::methods]
impl CharacterEffect {
    #[method(name = "GetTransform", args = 2)]
    pub fn get_transform(
        chr: crate::combat::character::Character,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetParticle", args = 2)]
    pub fn get_particle(
        chr: crate::combat::character::Character,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> crate::unity_engine::particlesystem::ParticleSystem;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, chr: crate::combat::character::Character) -> ();

    #[method(name = "SignalEffect", args = 2)]
    pub fn signal_effect(
        chr: crate::combat::character::Character,
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(
        self,
        prefab: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateHit", args = 3)]
    pub fn create_hit(
        self,
        tr: crate::combat::tr::TR,
        slash_type: crate::combat::slashtype::SlashType,
        effect_level: i32,
    ) -> ();

    #[method(name = "CreatePairingHit", args = 2)]
    pub fn create_pairing_hit(
        self,
        prefab: crate::unity_engine::gameobject::GameObject,
        tr: crate::combat::tr::TR,
    ) -> ();

    #[method(name = "GetHitEffectName", args = 2)]
    pub fn get_hit_effect_name(
        self,
        slash_type: crate::combat::slashtype::SlashType,
        level: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateGuard", args = 0)]
    pub fn create_guard(self) -> ();

    #[method(name = "CreateParry", args = 0)]
    pub fn create_parry(self) -> ();

    #[method(name = "CreateEfficacyHit", args = 0)]
    pub fn create_efficacy_hit(self) -> ();

    #[method(name = "CreateBreak", args = 0)]
    pub fn create_break(self) -> ();

    #[method(name = "CreateSmash", args = 0)]
    pub fn create_smash(self) -> ();
}

#[cfg(feature = "combat-charactereffect")]
impl CharacterEffect {
    pub fn new(chr: crate::combat::character::Character) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterEffect),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterEffectMethods>::ctor(this, chr);
        this
    }
}
