
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatviewer/CombatViewer.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatViewer")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CombatViewer {
    #[rename(name = "Settings")]
    pub settings: crate::combat::combatviewersettings::CombatViewerSettings,
    #[rename(name = "PlayAll")]
    pub play_all: bool,
    #[rename(name = "PlayAllIdx")]
    pub play_all_idx: i32,
}

#[cfg(feature = "combat-combatviewer")]
#[::unity2::methods]
impl CombatViewer {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::combat::combatviewer::CombatViewer;

    #[method(name = "set_Instance", args = 1)]
    pub fn set_instance(value: crate::combat::combatviewer::CombatViewer) -> ();

    #[method(name = "Restart", args = 0)]
    pub fn restart(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "combat-combatviewer")]
impl CombatViewer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatViewer),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatViewerMethods>::ctor(this);
        this
    }
}
