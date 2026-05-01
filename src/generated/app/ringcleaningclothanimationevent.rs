
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningclothanimationevent/RingCleaningClothAnimationEvent.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningClothAnimationEvent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingCleaningClothAnimationEvent {
    #[static_field]
    #[rename(name = "s_AnimTime")]
    pub s_anim_time: f32,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-ringcleaningclothanimationevent")]
#[::unity2::methods]
impl RingCleaningClothAnimationEvent {
    #[method(name = "get_IsPlayingAnim", args = 0)]
    pub fn get_is_playing_anim() -> bool;

    #[method(name = "set_IsPlayingAnim", args = 1)]
    pub fn set_is_playing_anim(value: bool) -> ();

    #[method(name = "get_CleaningLoopStartAction", args = 0)]
    pub fn get_cleaning_loop_start_action(self) -> crate::system::action::Action;

    #[method(name = "set_CleaningLoopStartAction", args = 1)]
    pub fn set_cleaning_loop_start_action(self, value: crate::system::action::Action) -> ();

    #[method(name = "get_StrongCleaningLoopStartAction", args = 0)]
    pub fn get_strong_cleaning_loop_start_action(self) -> crate::system::action::Action;

    #[method(name = "set_StrongCleaningLoopStartAction", args = 1)]
    pub fn set_strong_cleaning_loop_start_action(self, value: crate::system::action::Action) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "CleaningStartEvent", args = 0)]
    pub fn cleaning_start_event(self) -> ();

    #[method(name = "CleaningEndEvent", args = 0)]
    pub fn cleaning_end_event(self) -> ();

    #[method(name = "StrongCleaningStartEvent", args = 0)]
    pub fn strong_cleaning_start_event(self) -> ();

    #[method(name = "StrongCleaningEndEvent", args = 0)]
    pub fn strong_cleaning_end_event(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringcleaningclothanimationevent")]
impl RingCleaningClothAnimationEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningClothAnimationEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningClothAnimationEventMethods>::ctor(this);
        this
    }
}
