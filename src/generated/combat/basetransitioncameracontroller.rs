
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/basetransitioncameracontroller/BaseTransitionCameraController.md")))]
#[::unity2::class(namespace = "Combat", name = "BaseTransitionCameraController")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct BaseTransitionCameraController {
    #[rename(name = "ProgressTime")]
    pub progress_time: f32,
}

#[cfg(feature = "combat-basetransitioncameracontroller")]
#[::unity2::methods]
impl BaseTransitionCameraController {
    #[method(name = "get_Phase", args = 0)]
    pub fn get_phase(self) -> crate::combat::transitonphase::TransitonPhase;

    #[method(name = "set_Phase", args = 1)]
    pub fn set_phase(self, value: crate::combat::transitonphase::TransitonPhase) -> ();

    #[method(name = "get_TransitionTimeStart", args = 0)]
    pub fn get_transition_time_start(self) -> f32;

    #[method(name = "get_TransitionTimeReturn", args = 0)]
    pub fn get_transition_time_return(self) -> f32;

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "set_Progress", args = 1)]
    pub fn set_progress(self, value: f32) -> ();

    #[method(name = "get_TargetCamera", args = 0)]
    pub fn get_target_camera(self) -> crate::combat::basecameracontroller::BaseCameraController;

    #[method(name = "set_TargetCamera", args = 1)]
    pub fn set_target_camera(
        self,
        value: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "get_SourceCamera", args = 0)]
    pub fn get_source_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_SourceCamera", args = 1)]
    pub fn set_source_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "StartTransition", args = 1)]
    pub fn start_transition(
        self,
        next_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "ChangeTaget", args = 1)]
    pub fn change_taget(
        self,
        next_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "ReturnTransition", args = 1)]
    pub fn return_transition(
        self,
        next_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-basetransitioncameracontroller")]
impl BaseTransitionCameraController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseTransitionCameraController),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseTransitionCameraControllerMethods>::ctor(this);
        this
    }
}
