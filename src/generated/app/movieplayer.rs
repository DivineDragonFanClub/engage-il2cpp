
use crate::app::movieplayerbase::IMoviePlayerBase;
use crate::app::movieplayerbase::MoviePlayerBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/movieplayer/MoviePlayer.md")))]
#[::unity2::class(namespace = "App", name = "MoviePlayer")]
#[parent(crate::app::movieplayerbase::MoviePlayerBase)]
pub struct MoviePlayer {
    #[rename(name = "m_VideoPlayer")]
    pub m_video_player: crate::unity_engine::video::videoplayer::VideoPlayer,
    #[rename(name = "m_IsSoundEventEnable")]
    pub m_is_sound_event_enable: bool,
    #[rename(name = "m_triggerStop")]
    pub m_trigger_stop: bool,
}

#[cfg(feature = "app-movieplayer")]
#[::unity2::methods]
impl MoviePlayer {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        movie_file_name: ::unity2::Il2CppString,
        is_movie_file_name_direct: bool,
    ) -> ();

    #[method(name = "ErrorCallback", args = 2)]
    pub fn error_callback(
        self,
        player: crate::unity_engine::video::videoplayer::VideoPlayer,
        mess: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LoopPointReached", args = 1)]
    pub fn loop_point_reached(
        self,
        player: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "FrameReady", args = 2)]
    pub fn frame_ready(
        self,
        player: crate::unity_engine::video::videoplayer::VideoPlayer,
        frame_idx: i64,
    ) -> ();

    #[method(name = "SetupAfterLoadScene", args = 0)]
    pub fn setup_after_load_scene(self) -> bool;

    #[method(name = "SetupByMovieCanvasPrefab", args = 1)]
    pub fn setup_by_movie_canvas_prefab(
        self,
        canvas_prefab: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetMovieFilePathToVideoPlayer", args = 0)]
    pub fn set_movie_file_path_to_video_player(self) -> bool;

    #[method(name = "EnableSoundEvent", args = 0)]
    pub fn enable_sound_event(self) -> ();

    #[method(name = "DisableSoundEvent", args = 0)]
    pub fn disable_sound_event(self) -> ();

    #[method(name = "IsTruePlayEnd", args = 0)]
    pub fn is_true_play_end(self) -> bool;

    #[method(name = "GetScreenObjectName", args = 0)]
    pub fn get_screen_object_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsPaused", args = 0)]
    pub fn is_paused(self) -> bool;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "IsPlayEnd", args = 0)]
    pub fn is_play_end(self) -> bool;

    #[method(name = "GetMovieLength", args = 0)]
    pub fn get_movie_length(self) -> f32;

    #[method(name = "GetPlayingPosition", args = 0)]
    pub fn get_playing_position(self) -> f32;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "IsPrepared", args = 0)]
    pub fn is_prepared(self) -> bool;

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "SuspendOn", args = 0)]
    pub fn suspend_on(self) -> ();

    #[method(name = "SuspendOff", args = 0)]
    pub fn suspend_off(self) -> ();
}

#[cfg(feature = "app-movieplayer")]
impl MoviePlayer {
    pub fn new(movie_file_name: ::unity2::Il2CppString, is_movie_file_name_direct: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MoviePlayer),
                ::core::stringify!(new),
            )
        });
        <Self as IMoviePlayerMethods>::ctor(this, movie_file_name, is_movie_file_name_direct);
        this
    }
}
