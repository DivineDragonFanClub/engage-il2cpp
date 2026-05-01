
use crate::combat::characterlying::CharacterLying;
use crate::combat::characterlying::ICharacterLying;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterlyinghuman/CharacterLyingHuman.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterLyingHuman")]
#[parent(crate::combat::characterlying::CharacterLying)]
pub struct CharacterLyingHuman {
    #[rename(name = "m_StartPos")]
    pub m_start_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_EndPos")]
    pub m_end_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_EndTime")]
    pub m_end_time: f32,
    #[rename(name = "m_BodyRotation")]
    pub m_body_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_LastNrm")]
    pub m_last_nrm: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_LastY")]
    pub m_last_y: f32,
    #[rename(name = "m_Skipped")]
    pub m_skipped: bool,
    #[rename(name = "m_SkipFrames")]
    pub m_skip_frames: i32,
}

#[cfg(feature = "combat-characterlyinghuman")]
#[::unity2::methods]
impl CharacterLyingHuman {
    #[method(name = "get_IsInAir", args = 0)]
    pub fn get_is_in_air(self) -> bool;

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "MyStart", args = 1)]
    pub fn my_start(self, die_hash: i32) -> ();

    #[method(name = "MakeMyDeadPose", args = 0)]
    pub fn make_my_dead_pose(self) -> ();

    #[method(name = "MyLateUpdate", args = 0)]
    pub fn my_late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterlyinghuman")]
impl CharacterLyingHuman {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterLyingHuman),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterLyingHumanMethods>::ctor(this);
        this
    }
}
