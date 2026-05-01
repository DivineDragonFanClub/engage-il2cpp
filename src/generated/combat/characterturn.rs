
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterturn/CharacterTurn.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterTurn")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterTurn {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_StartDir")]
    pub m_start_dir: crate::combat::fxz::FXZ,
    #[rename(name = "m_TargetDir")]
    pub m_target_dir: crate::combat::fxz::FXZ,
    #[rename(name = "m_Ratio")]
    pub m_ratio: f32,
}

#[cfg(feature = "combat-characterturn")]
#[::unity2::methods]
impl CharacterTurn {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "SetToEnemy", args = 0)]
    pub fn set_to_enemy(self) -> ();

    #[method(name = "SetDir", args = 1)]
    pub fn set_dir(self, target_dir: crate::combat::fxz::FXZ) -> ();

    #[method(name = "TurnToEnemy", args = 0)]
    pub fn turn_to_enemy(self) -> ();

    #[method(name = "TurnToDir", args = 1)]
    pub fn turn_to_dir(self, target_dir: crate::combat::fxz::FXZ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterturn")]
impl CharacterTurn {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterTurn),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterTurnMethods>::ctor(this);
        this
    }
}
