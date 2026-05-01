
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/telopinstance/TelopInstance.md")))]
#[::unity2::class(namespace = "App", name = "TelopInstance")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TelopInstance {
    #[rename(name = "m_IsKeyWait")]
    pub m_is_key_wait: bool,
    #[rename(name = "m_StartSoundEvent")]
    pub m_start_sound_event: ::unity2::Il2CppString,
    #[rename(name = "m_KeySoundEvent")]
    pub m_key_sound_event: ::unity2::Il2CppString,
    #[rename(name = "m_SkipableTime")]
    pub m_skipable_time: f32,
    #[rename(name = "m_MvpWaitTime")]
    pub m_mvp_wait_time: f32,
    #[static_field]
    #[rename(name = "AnimeLayerMain")]
    pub anime_layer_main: i32,
    #[static_field]
    #[rename(name = "AnimeLayerLoopColor")]
    pub anime_layer_loop_color: i32,
    #[static_field]
    #[rename(name = "AnimeLayerLoopSRT")]
    pub anime_layer_loop_srt: i32,
    #[static_field]
    #[rename(name = "FadeTime")]
    pub fade_time: f32,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CanvasGroup")]
    pub m_canvas_group: crate::unity_engine::canvasgroup::CanvasGroup,
    #[rename(name = "m_FadeTime")]
    pub m_fade_time: f32,
    #[rename(name = "m_ElapsedTime")]
    pub m_elapsed_time: f32,
    #[rename(name = "m_IsKeyLock")]
    pub m_is_key_lock: bool,
    #[rename(name = "m_IsForcedPlayOut")]
    pub m_is_forced_play_out: bool,
}

#[cfg(feature = "app-telopinstance")]
#[::unity2::methods]
impl TelopInstance {
    #[method(name = "SetKeyLock", args = 0)]
    pub fn set_key_lock(self) -> ();

    #[method(name = "PlayOut", args = 0)]
    pub fn play_out(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-telopinstance")]
impl TelopInstance {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TelopInstance),
                ::core::stringify!(new),
            )
        });
        <Self as ITelopInstanceMethods>::ctor(this);
        this
    }
}
