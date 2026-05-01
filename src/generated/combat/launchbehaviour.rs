
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/launchbehaviour/LaunchBehaviour.md")))]
#[::unity2::class(namespace = "Combat", name = "LaunchBehaviour")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct LaunchBehaviour {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
}

#[cfg(feature = "combat-launchbehaviour")]
#[::unity2::methods]
impl LaunchBehaviour {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_WeaponInstance", args = 0)]
    pub fn get_weapon_instance(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_WeaponInstance", args = 1)]
    pub fn set_weapon_instance(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "OnCharacterSetup", args = 1)]
    pub fn on_character_setup(self, owner: crate::combat::character::Character) -> ();

    #[method(name = "OnEnterAttack", args = 0)]
    pub fn on_enter_attack(self) -> ();

    #[method(name = "OnHitTimePredicted", args = 1)]
    pub fn on_hit_time_predicted(self, world_hit_time: f32) -> ();

    #[method(name = "RecalcFlyingTime", args = 0)]
    pub fn recalc_flying_time(self) -> ();

    #[method(name = "get_FlyingTime", args = 0)]
    pub fn get_flying_time(self) -> f32;

    #[method(name = "set_FlyingTime", args = 1)]
    pub fn set_flying_time(self, value: f32) -> ();

    #[method(name = "get_WorldHitTime", args = 0)]
    pub fn get_world_hit_time(self) -> f32;

    #[method(name = "set_WorldHitTime", args = 1)]
    pub fn set_world_hit_time(self, value: f32) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "MakeAvoidObserver", args = 2)]
    pub fn make_avoid_observer(
        self,
        chr: crate::combat::character::Character,
        time_to_hit: f32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-launchbehaviour")]
impl LaunchBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LaunchBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as ILaunchBehaviourMethods>::ctor(this);
        this
    }
}
