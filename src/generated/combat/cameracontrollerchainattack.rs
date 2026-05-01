
use crate::combat::basecameracontroller::BaseCameraController;
use crate::combat::basecameracontroller::IBaseCameraController;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerchainattack/CameraControllerChainAttack.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerChainAttack")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerChainAttack {
    #[rename(name = "Distance")]
    pub distance: f32,
    #[rename(name = "EnemyDistance")]
    pub enemy_distance: f32,
    #[rename(name = "Height")]
    pub height: f32,
    #[rename(name = "Back")]
    pub back: f32,
}

#[cfg(feature = "combat-cameracontrollerchainattack")]
#[::unity2::methods]
impl CameraControllerChainAttack {
    #[method(name = "get_IsEnemyAtk", args = 0)]
    pub fn get_is_enemy_atk(self) -> bool;

    #[method(name = "set_IsEnemyAtk", args = 1)]
    pub fn set_is_enemy_atk(self, value: bool) -> ();

    #[method(name = "get_IsInverse", args = 0)]
    pub fn get_is_inverse(self) -> bool;

    #[method(name = "set_IsInverse", args = 1)]
    pub fn set_is_inverse(self, value: bool) -> ();

    #[method(name = "SetInverse", args = 2)]
    pub fn set_inverse(self, inv_side: bool, inv_camera: bool) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerchainattack")]
impl CameraControllerChainAttack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerChainAttack),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerChainAttackMethods>::ctor(this);
        this
    }
}
