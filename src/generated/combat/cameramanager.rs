
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameramanager/CameraManager.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraManager")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: combat :: cameramanager :: CameraManager >)]
pub struct CameraManager {
    #[rename(name = "Switch")]
    pub switch: crate::combat::cameraswitch::CameraSwitch,
    #[rename(name = "PosData")]
    pub pos_data: crate::combat::camerapositiondata::CameraPositionData,
    #[rename(name = "Selector")]
    pub selector: crate::combat::camerasituationconverter::CameraSituationConverter,
    #[rename(name = "m_LastUnusedTime")]
    pub m_last_unused_time: f32,
    #[rename(name = "m_IsTemporaryCamera")]
    pub m_is_temporary_camera: bool,
    #[rename(name = "m_LastChangeTime")]
    pub m_last_change_time: f32,
}

#[cfg(feature = "combat-cameramanager")]
#[::unity2::methods]
impl CameraManager {
    #[method(name = "get_TransitionProgress", args = 0)]
    pub fn get_transition_progress(self) -> f32;

    #[method(name = "set_TransitionProgress", args = 1)]
    pub fn set_transition_progress(self, value: f32) -> ();

    #[method(name = "get_IsPositionCrossed", args = 0)]
    pub fn get_is_position_crossed(self) -> bool;

    #[method(name = "IsTargeting", args = 1)]
    pub fn is_targeting(self, side: i32) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "BeginCamera", args = 2)]
    pub fn begin_camera(
        self,
        record: crate::combat::combatrecord::CombatRecord,
        transition: bool,
    ) -> ();

    #[method(name = "BeginCamera", args = 2)]
    pub fn begin_camera_2(
        self,
        chara: ::unity2::Array<crate::combat::character::Character>,
        transition: bool,
    ) -> ();

    #[method(name = "EndCamera", args = 2)]
    pub fn end_camera(
        self,
        transition: bool,
        camera_mode: crate::app::viewmode::ViewMode_Mode,
    ) -> ();

    #[method(name = "ReverseCamera", args = 0)]
    pub fn reverse_camera(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ReverseMainCamera", args = 0)]
    pub fn reverse_main_camera(self) -> ();

    #[method(name = "UpdateSituation", args = 2)]
    pub fn update_situation(
        self,
        change_style: crate::combat::camerasituation::CameraSituation,
        change_force: bool,
    ) -> ();

    #[method(name = "UpdateSituation", args = 2)]
    pub fn update_situation_2(
        self,
        ev: crate::unity_engine::animationevent::AnimationEvent,
        call_side: i32,
    ) -> ();

    #[method(name = "SwitchCamera", args = 1)]
    pub fn switch_camera(self, pos: crate::combat::cameraposition::CameraPosition) -> ();

    #[method(name = "StartShake", args = 3)]
    pub fn start_shake(
        self,
        magnitude: f32,
        source_pos: crate::unity_engine::vector3::Vector3,
        is_critical: bool,
    ) -> ();

    #[method(name = "ArmorShake", args = 2)]
    pub fn armor_shake(
        self,
        magnitude: f32,
        source_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Stabilize", args = 0)]
    pub fn stabilize(self) -> ();

    #[method(name = "BeginTransition", args = 1)]
    pub fn begin_transition(self, camera_pos: crate::combat::cameraposition::CameraPosition) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameramanager")]
impl CameraManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraManager),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraManagerMethods>::ctor(this);
        this
    }
}
