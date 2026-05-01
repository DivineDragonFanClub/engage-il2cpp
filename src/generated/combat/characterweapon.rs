
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterweapon/CharacterWeapon.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterWeapon")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterWeapon {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[static_field]
    #[rename(name = "Right")]
    pub right: i32,
    #[static_field]
    #[rename(name = "Left")]
    pub left: i32,
    #[static_field]
    #[rename(name = "NumHands")]
    pub num_hands: i32,
    #[rename(name = "m_Hands")]
    pub m_hands: ::unity2::Array<crate::combat::trailhand::TrailHand>,
}

#[cfg(feature = "combat-characterweapon")]
#[::unity2::methods]
impl CharacterWeapon {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "MyLateUpdate", args = 0)]
    pub fn my_late_update(self) -> ();

    #[method(name = "Play", args = 3)]
    pub fn play(
        cp: crate::combat::character::Character,
        name: ::unity2::Il2CppString,
        duration: f32,
    ) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterweapon")]
impl CharacterWeapon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterWeapon),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterWeaponMethods>::ctor(this);
        this
    }
}
