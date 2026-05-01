
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterikfoot/CharacterIKFoot.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterIKFoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterIKFoot {
    #[rename(name = "m_Race")]
    pub m_race: crate::combat::animname::AnimName_Race,
    #[rename(name = "m_BodyIK")]
    pub m_body_ik: crate::unity_engine::behaviour::Behaviour,
    #[rename(name = "m_RideIK")]
    pub m_ride_ik: crate::unity_engine::behaviour::Behaviour,
    #[rename(name = "limbIKs")]
    pub limb_i_ks: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::behaviour::Behaviour,
    >,
}

#[cfg(feature = "combat-characterikfoot")]
#[::unity2::methods]
impl CharacterIKFoot {
    #[method(name = "Setup", args = 1)]
    pub fn setup(self, race: crate::combat::animname::AnimName_Race) -> ();

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, value: bool) -> ();

    #[method(name = "DeactivateIfHorse", args = 0)]
    pub fn deactivate_if_horse(self) -> ();

    #[method(name = "DeactivateIfDragonic", args = 0)]
    pub fn deactivate_if_dragonic(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterikfoot")]
impl CharacterIKFoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterIKFoot),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterIKFootMethods>::ctor(this);
        this
    }
}
