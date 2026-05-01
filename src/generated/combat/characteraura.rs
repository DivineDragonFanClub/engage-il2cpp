
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characteraura/CharacterAura.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterAura")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterAura {
    #[rename(name = "m_AppearEffect")]
    pub m_appear_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DisappearEffect")]
    pub m_disappear_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "AppearDuration")]
    pub appear_duration: f32,
    #[rename(name = "DisappearDuration")]
    pub disappear_duration: f32,
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_IsAppear")]
    pub m_is_appear: bool,
    #[rename(name = "m_DisappearDelayTime")]
    pub m_disappear_delay_time: f32,
}

#[cfg(feature = "combat-characteraura")]
#[::unity2::methods]
impl CharacterAura {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "get_LastDisappearPosition", args = 0)]
    pub fn get_last_disappear_position(self) -> crate::combat::fxz::FXZ;

    #[method(name = "set_LastDisappearPosition", args = 1)]
    pub fn set_last_disappear_position(self, value: crate::combat::fxz::FXZ) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "SetAppear", args = 1)]
    pub fn set_appear(self, visible: bool) -> ();

    #[method(name = "Appear", args = 1)]
    pub fn appear(self, is_end_of_combat: bool) -> ();

    #[method(name = "RelocateToSpace", args = 0)]
    pub fn relocate_to_space(self) -> ();

    #[method(name = "RelocateForCombatEnd", args = 0)]
    pub fn relocate_for_combat_end(self) -> ();

    #[method(name = "Disappear", args = 2)]
    pub fn disappear(self, delay_time: f32, silent: bool) -> ();

    #[method(name = "PlayTriggerEffect", args = 1)]
    pub fn play_trigger_effect(self, fx: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characteraura")]
impl CharacterAura {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterAura),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterAuraMethods>::ctor(this);
        this
    }
}
