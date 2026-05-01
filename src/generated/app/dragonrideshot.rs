
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideshot/DragonRideShot.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideShot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideShot {
    #[rename(name = "m_PrePosition")]
    pub m_pre_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_IsAssist")]
    pub m_is_assist: bool,
    #[rename(name = "m_IsPenetrate")]
    pub m_is_penetrate: bool,
    #[rename(name = "m_IsSpecialShot")]
    pub m_is_special_shot: bool,
    #[rename(name = "m_IsMaximumAssist")]
    pub m_is_maximum_assist: bool,
    #[rename(name = "m_IsDestroy")]
    pub m_is_destroy: bool,
    #[rename(name = "m_DiffInterp")]
    pub m_diff_interp: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_Effect")]
    pub m_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CheckRayPoint")]
    pub m_check_ray_point: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_HitCheckRay")]
    pub m_hit_check_ray: crate::unity_engine::ray::Ray,
    #[rename(name = "m_LifeTimer")]
    pub m_life_timer: f32,
}

#[cfg(feature = "app-dragonrideshot")]
#[::unity2::methods]
impl DragonRideShot {
    #[method(name = "get_EraseSecond", args = 0)]
    pub fn get_erase_second(self) -> f32;

    #[method(name = "set_EraseSecond", args = 1)]
    pub fn set_erase_second(self, value: f32) -> ();

    #[method(name = "get_ShotSpeed", args = 0)]
    pub fn get_shot_speed(self) -> f32;

    #[method(name = "set_ShotSpeed", args = 1)]
    pub fn set_shot_speed(self, value: f32) -> ();

    #[method(name = "get_IsInitialized", args = 0)]
    pub fn get_is_initialized(self) -> bool;

    #[method(name = "set_IsInitialized", args = 1)]
    pub fn set_is_initialized(self, value: bool) -> ();

    #[method(name = "get_StraightVector", args = 0)]
    pub fn get_straight_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_StraightVector", args = 1)]
    pub fn set_straight_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_FirstVector", args = 0)]
    pub fn get_first_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_FirstVector", args = 1)]
    pub fn set_first_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_DiffVector", args = 0)]
    pub fn get_diff_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_DiffVector", args = 1)]
    pub fn set_diff_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_InterpStraightSecond", args = 0)]
    pub fn get_interp_straight_second(self) -> f32;

    #[method(name = "set_InterpStraightSecond", args = 1)]
    pub fn set_interp_straight_second(self, value: f32) -> ();

    #[method(name = "Initialize", args = 4)]
    pub fn initialize(
        self,
        is_assist: bool,
        is_special: bool,
        is_penetrate: bool,
        is_maximum: bool,
    ) -> ();

    #[method(name = "PostUpdate", args = 0)]
    pub fn post_update(self) -> ();

    #[method(name = "OnTriggerEnter", args = 1)]
    pub fn on_trigger_enter(self, collision: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "CallReset", args = 0)]
    pub fn call_reset(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonrideshot")]
impl DragonRideShot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideShot),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideShotMethods>::ctor(this);
        this
    }
}
