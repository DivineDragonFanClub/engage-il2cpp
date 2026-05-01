
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningcloth/RingCleaningCloth.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningCloth")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingCleaningCloth {
    #[rename(name = "m_HelpObject")]
    pub m_help_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Rect")]
    pub m_rect: crate::unity_engine::recttransform::RectTransform,
    #[static_field]
    #[rename(name = "WidthRatio")]
    pub width_ratio: f32,
    #[static_field]
    #[rename(name = "HeightRatio")]
    pub height_ratio: f32,
    #[static_field]
    #[rename(name = "MoveSpeed")]
    pub move_speed: f32,
    #[static_field]
    #[rename(name = "MoveSpeedAccelRate")]
    pub move_speed_accel_rate: f32,
    #[rename(name = "m_LocalPosition")]
    pub m_local_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_HoldGapPosition")]
    pub m_hold_gap_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_IsStartAnim")]
    pub m_is_start_anim: bool,
    #[rename(name = "m_OldAnimSpeed")]
    pub m_old_anim_speed: f32,
}

#[cfg(feature = "app-ringcleaningcloth")]
#[::unity2::methods]
impl RingCleaningCloth {
    #[method(name = "get_Rect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "get_IsHitRing", args = 0)]
    pub fn get_is_hit_ring(self) -> bool;

    #[method(name = "set_IsHitRing", args = 1)]
    pub fn set_is_hit_ring(self, value: bool) -> ();

    #[method(name = "get_HitCollider", args = 0)]
    pub fn get_hit_collider(self) -> crate::unity_engine::collider::Collider;

    #[method(name = "set_HitCollider", args = 1)]
    pub fn set_hit_collider(self, value: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "get_HitVector", args = 0)]
    pub fn get_hit_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_HitVector", args = 1)]
    pub fn set_hit_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_IsPossibleCleaning", args = 0)]
    pub fn get_is_possible_cleaning(self) -> bool;

    #[method(name = "set_IsPossibleCleaning", args = 1)]
    pub fn set_is_possible_cleaning(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateMove", args = 4)]
    pub fn update_move(
        self,
        lsx: f32,
        lsy: f32,
        is_speed_up: bool,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "UpdateMoveWithGap", args = 0)]
    pub fn update_move_with_gap(self) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "ShowHelp", args = 0)]
    pub fn show_help(self) -> ();

    #[method(name = "HideHelp", args = 0)]
    pub fn hide_help(self) -> ();

    #[method(name = "IsHitRay", args = 1)]
    pub fn is_hit_ray(self, camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "IsHitRayImpl", args = 3)]
    pub fn is_hit_ray_impl(
        self,
        camera: crate::unity_engine::camera::Camera,
        collider_hit_pos: crate::unity_engine::vector3::Vector3,
        collider: crate::unity_engine::collider::Collider,
    ) -> bool;

    #[method(name = "PlayCleaningAnim", args = 1)]
    pub fn play_cleaning_anim(
        self,
        strength: crate::app::ringcleaningsequence::RingCleaningSequence_Strength,
    ) -> ();

    #[method(name = "StartCleaningAnim", args = 1)]
    pub fn start_cleaning_anim(self, anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopCleaningAnim", args = 0)]
    pub fn stop_cleaning_anim(self) -> ();

    #[method(name = "IsPlayCleaningAnim", args = 0)]
    pub fn is_play_cleaning_anim(self) -> bool;

    #[method(name = "IsPlayingCleaningAnim", args = 1)]
    pub fn is_playing_cleaning_anim(self, use_correct: bool) -> bool;

    #[method(name = "GetAnimatorNormalizeTime", args = 0)]
    pub fn get_animator_normalize_time(self) -> f32;

    #[method(name = "GetHitPart", args = 0)]
    pub fn get_hit_part(self) -> crate::app::ringcolliderpart::RingColliderPart;

    #[method(name = "PauseAnim", args = 0)]
    pub fn pause_anim(self) -> ();

    #[method(name = "SetGap", args = 2)]
    pub fn set_gap(self, x: f32, y: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringcleaningcloth")]
impl RingCleaningCloth {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningCloth),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningClothMethods>::ctor(this);
        this
    }
}
