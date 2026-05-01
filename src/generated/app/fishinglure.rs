
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinglure/FishingLure.md")))]
#[::unity2::class(namespace = "App", name = "FishingLure")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FishingLure {}

#[cfg(feature = "app-fishinglure")]
#[::unity2::methods]
impl FishingLure {
    #[method(name = "get_HitAreaName", args = 0)]
    pub fn get_hit_area_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HitAreaName", args = 1)]
    pub fn set_hit_area_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "OnCollisionEnter", args = 1)]
    pub fn on_collision_enter(self, collision: crate::unity_engine::collision::Collision) -> ();

    #[method(name = "OnCollisionStay", args = 1)]
    pub fn on_collision_stay(self, collision: crate::unity_engine::collision::Collision) -> ();

    #[method(name = "OnTriggerEnter", args = 1)]
    pub fn on_trigger_enter(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "OnTriggerStay", args = 1)]
    pub fn on_trigger_stay(self, other: crate::unity_engine::collider::Collider) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishinglure")]
impl FishingLure {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingLure),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingLureMethods>::ctor(this);
        this
    }
}
