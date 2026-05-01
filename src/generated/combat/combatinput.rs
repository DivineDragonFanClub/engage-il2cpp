
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatinput/CombatInput.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatInput")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CombatInput {}

#[cfg(feature = "combat-combatinput")]
#[::unity2::methods]
impl CombatInput {
    #[method(name = "get_IsEnabled", args = 0)]
    pub fn get_is_enabled(self) -> bool;

    #[method(name = "get_CameraPan", args = 0)]
    pub fn get_camera_pan(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_CameraPan", args = 1)]
    pub fn set_camera_pan(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_CameraTruck", args = 0)]
    pub fn get_camera_truck(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_CameraTruck", args = 1)]
    pub fn set_camera_truck(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_KeepAForDebug", args = 0)]
    pub fn get_keep_a_for_debug(self) -> bool;

    #[method(name = "set_KeepAForDebug", args = 1)]
    pub fn set_keep_a_for_debug(self, value: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-combatinput")]
impl CombatInput {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatInput),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatInputMethods>::ctor(this);
        this
    }
}
