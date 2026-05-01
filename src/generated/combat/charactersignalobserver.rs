
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactersignalobserver/CharacterSignalObserver.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterSignalObserver")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterSignalObserver {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "_radialBlur")]
    pub radial_blur:
        crate::unity_engine::rendering::universal::custom::customradialblur::CustomRadialBlur,
    #[rename(name = "pushedFootIKEnabled")]
    pub pushed_foot_ik_enabled: bool,
}

#[cfg(feature = "combat-charactersignalobserver")]
#[::unity2::methods]
impl CharacterSignalObserver {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_RadialBlur", args = 0)]
    pub fn get_radial_blur(
        self,
    ) -> crate::unity_engine::rendering::universal::custom::customradialblur::CustomRadialBlur;

    #[method(name = "FindVolumeFromScene", args = 0)]
    pub fn find_volume_from_scene() -> crate::unity_engine::rendering::volume::Volume;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "PushAndEnableFootIK", args = 1)]
    pub fn push_and_enable_foot_ik(self, v: bool) -> ();

    #[method(name = "PopFootIK", args = 0)]
    pub fn pop_foot_ik(self) -> ();

    #[method(name = "GroundParticle", args = 2)]
    pub fn ground_particle(self, effect_type: i32, node_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopFootstepObservers", args = 0)]
    pub fn stop_footstep_observers(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactersignalobserver")]
impl CharacterSignalObserver {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterSignalObserver),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterSignalObserverMethods>::ctor(this);
        this
    }
}
