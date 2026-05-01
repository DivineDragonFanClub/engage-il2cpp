
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameraswitch/CameraSwitch.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraSwitch")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CameraSwitch {
    #[rename(name = "MainCamera")]
    pub main_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "VCameras")]
    pub v_cameras:
        crate::system::collections::generic::list_1::List_1<crate::combat::camerainfo::CameraInfo>,
    #[rename(name = "m_MaxPriprity")]
    pub m_max_priprity: i32,
}

#[cfg(feature = "combat-cameraswitch")]
#[::unity2::methods]
impl CameraSwitch {
    #[method(name = "get_SourceCamera", args = 0)]
    pub fn get_source_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_SourceCamera", args = 1)]
    pub fn set_source_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_CurrentCamera", args = 0)]
    pub fn get_current_camera(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "set_CurrentCamera", args = 1)]
    pub fn set_current_camera(self, value: crate::combat::cameraposition::CameraPosition) -> ();

    #[method(name = "get_CurrentInterruptCamera", args = 0)]
    pub fn get_current_interrupt_camera(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "set_CurrentInterruptCamera", args = 1)]
    pub fn set_current_interrupt_camera(
        self,
        value: crate::combat::cameraposition::CameraPosition,
    ) -> ();

    #[method(name = "get_IsInterrupting", args = 0)]
    pub fn get_is_interrupting(self) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        pos: crate::combat::cameraposition::CameraPosition,
    ) -> crate::combat::basecameracontroller::BaseCameraController;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "SwitchCamera", args = 2)]
    pub fn switch_camera(
        self,
        next_camera: crate::combat::cameraposition::CameraPosition,
        force: bool,
    ) -> ();

    #[method(name = "StartTransition", args = 1)]
    pub fn start_transition(
        self,
        next_camera: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameraswitch")]
impl CameraSwitch {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraSwitch),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraSwitchMethods>::ctor(this);
        this
    }
}
