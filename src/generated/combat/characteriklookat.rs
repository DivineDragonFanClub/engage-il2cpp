
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characteriklookat/CharacterIKLookAt.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterIKLookAt")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterIKLookAt {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_LookTargetGO")]
    pub m_look_target_go: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "combat-characteriklookat")]
#[::unity2::methods]
impl CharacterIKLookAt {
    #[method(name = "get_FixedLookTargetHeight", args = 0)]
    pub fn get_fixed_look_target_height(self) -> bool;

    #[method(name = "set_FixedLookTargetHeight", args = 1)]
    pub fn set_fixed_look_target_height(self, value: bool) -> ();

    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "MyLateUpdate", args = 0)]
    pub fn my_late_update(self) -> ();

    #[method(name = "SetWeight", args = 1)]
    pub fn set_weight(self, w: f32) -> ();

    #[method(name = "Skip", args = 0)]
    pub fn skip(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characteriklookat")]
impl CharacterIKLookAt {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterIKLookAt),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterIKLookAtMethods>::ctor(this);
        this
    }
}
