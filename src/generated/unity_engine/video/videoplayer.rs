
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/video/videoplayer/VideoPlayer_FrameReadyEventHandler.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Video",
    name = "VideoPlayer.FrameReadyEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VideoPlayer_FrameReadyEventHandler {}

#[cfg(feature = "unity_engine-video-videoplayer")]
#[::unity2::methods]
impl VideoPlayer_FrameReadyEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        frame_idx: i64,
    ) -> ();
}

#[cfg(feature = "unity_engine-video-videoplayer")]
impl VideoPlayer_FrameReadyEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VideoPlayer_FrameReadyEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IVideoPlayer_FrameReadyEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/video/videoplayer/VideoPlayer.md")))]
#[::unity2::class(namespace = "UnityEngine.Video", name = "VideoPlayer")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct VideoPlayer {
    #[rename(name = "prepareCompleted")]
    pub prepare_completed: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    #[rename(name = "loopPointReached")]
    pub loop_point_reached: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    #[rename(name = "started")]
    pub started: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    #[rename(name = "frameDropped")]
    pub frame_dropped: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    #[rename(name = "errorReceived")]
    pub error_received: crate::unity_engine::video::videoplayer::VideoPlayer_ErrorEventHandler,
    #[rename(name = "seekCompleted")]
    pub seek_completed: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    #[rename(name = "clockResyncOccurred")]
    pub clock_resync_occurred:
        crate::unity_engine::video::videoplayer::VideoPlayer_TimeEventHandler,
    #[rename(name = "frameReady")]
    pub frame_ready: crate::unity_engine::video::videoplayer::VideoPlayer_FrameReadyEventHandler,
}

#[cfg(feature = "unity_engine-video-videoplayer")]
#[::unity2::methods]
impl VideoPlayer {
    #[method(name = "get_source", args = 0)]
    pub fn get_source(self) -> crate::unity_engine::video::videosource::VideoSource;

    #[method(name = "set_source", args = 1)]
    pub fn set_source(self, value: crate::unity_engine::video::videosource::VideoSource) -> ();

    #[method(name = "get_url", args = 0)]
    pub fn get_url(self) -> ::unity2::Il2CppString;

    #[method(name = "set_url", args = 1)]
    pub fn set_url(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_clip", args = 0)]
    pub fn get_clip(self) -> crate::unity_engine::video::videoclip::VideoClip;

    #[method(name = "set_clip", args = 1)]
    pub fn set_clip(self, value: crate::unity_engine::video::videoclip::VideoClip) -> ();

    #[method(name = "get_renderMode", args = 0)]
    pub fn get_render_mode(self) -> crate::unity_engine::video::videorendermode::VideoRenderMode;

    #[method(name = "set_renderMode", args = 1)]
    pub fn set_render_mode(
        self,
        value: crate::unity_engine::video::videorendermode::VideoRenderMode,
    ) -> ();

    #[method(name = "get_targetCamera", args = 0)]
    pub fn get_target_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_targetCamera", args = 1)]
    pub fn set_target_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_targetTexture", args = 0)]
    pub fn get_target_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "set_targetTexture", args = 1)]
    pub fn set_target_texture(self, value: crate::unity_engine::rendertexture::RenderTexture)
        -> ();

    #[method(name = "get_targetMaterialRenderer", args = 0)]
    pub fn get_target_material_renderer(self) -> crate::unity_engine::renderer::Renderer;

    #[method(name = "set_targetMaterialRenderer", args = 1)]
    pub fn set_target_material_renderer(self, value: crate::unity_engine::renderer::Renderer)
        -> ();

    #[method(name = "get_targetMaterialProperty", args = 0)]
    pub fn get_target_material_property(self) -> ::unity2::Il2CppString;

    #[method(name = "set_targetMaterialProperty", args = 1)]
    pub fn set_target_material_property(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_aspectRatio", args = 0)]
    pub fn get_aspect_ratio(self)
        -> crate::unity_engine::video::videoaspectratio::VideoAspectRatio;

    #[method(name = "set_aspectRatio", args = 1)]
    pub fn set_aspect_ratio(
        self,
        value: crate::unity_engine::video::videoaspectratio::VideoAspectRatio,
    ) -> ();

    #[method(name = "get_targetCameraAlpha", args = 0)]
    pub fn get_target_camera_alpha(self) -> f32;

    #[method(name = "set_targetCameraAlpha", args = 1)]
    pub fn set_target_camera_alpha(self, value: f32) -> ();

    #[method(name = "get_targetCamera3DLayout", args = 0)]
    pub fn get_target_camera3_d_layout(
        self,
    ) -> crate::unity_engine::video::video3dlayout::Video3DLayout;

    #[method(name = "set_targetCamera3DLayout", args = 1)]
    pub fn set_target_camera3_d_layout(
        self,
        value: crate::unity_engine::video::video3dlayout::Video3DLayout,
    ) -> ();

    #[method(name = "get_texture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "get_isPrepared", args = 0)]
    pub fn get_is_prepared(self) -> bool;

    #[method(name = "get_waitForFirstFrame", args = 0)]
    pub fn get_wait_for_first_frame(self) -> bool;

    #[method(name = "set_waitForFirstFrame", args = 1)]
    pub fn set_wait_for_first_frame(self, value: bool) -> ();

    #[method(name = "get_playOnAwake", args = 0)]
    pub fn get_play_on_awake(self) -> bool;

    #[method(name = "set_playOnAwake", args = 1)]
    pub fn set_play_on_awake(self, value: bool) -> ();

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> ();

    #[method(name = "Pause", args = 0)]
    pub fn pause(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "get_isPlaying", args = 0)]
    pub fn get_is_playing(self) -> bool;

    #[method(name = "get_isPaused", args = 0)]
    pub fn get_is_paused(self) -> bool;

    #[method(name = "get_canSetTime", args = 0)]
    pub fn get_can_set_time(self) -> bool;

    #[method(name = "get_time", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "set_time", args = 1)]
    pub fn set_time(self, value: f64) -> ();

    #[method(name = "get_frame", args = 0)]
    pub fn get_frame(self) -> i64;

    #[method(name = "set_frame", args = 1)]
    pub fn set_frame(self, value: i64) -> ();

    #[method(name = "get_clockTime", args = 0)]
    pub fn get_clock_time(self) -> f64;

    #[method(name = "get_canStep", args = 0)]
    pub fn get_can_step(self) -> bool;

    #[method(name = "StepForward", args = 0)]
    pub fn step_forward(self) -> ();

    #[method(name = "get_canSetPlaybackSpeed", args = 0)]
    pub fn get_can_set_playback_speed(self) -> bool;

    #[method(name = "get_playbackSpeed", args = 0)]
    pub fn get_playback_speed(self) -> f32;

    #[method(name = "set_playbackSpeed", args = 1)]
    pub fn set_playback_speed(self, value: f32) -> ();

    #[method(name = "get_isLooping", args = 0)]
    pub fn get_is_looping(self) -> bool;

    #[method(name = "set_isLooping", args = 1)]
    pub fn set_is_looping(self, value: bool) -> ();

    #[method(name = "get_canSetTimeSource", args = 0)]
    pub fn get_can_set_time_source(self) -> bool;

    #[method(name = "get_timeSource", args = 0)]
    pub fn get_time_source(self) -> crate::unity_engine::video::videotimesource::VideoTimeSource;

    #[method(name = "set_timeSource", args = 1)]
    pub fn set_time_source(
        self,
        value: crate::unity_engine::video::videotimesource::VideoTimeSource,
    ) -> ();

    #[method(name = "get_timeReference", args = 0)]
    pub fn get_time_reference(
        self,
    ) -> crate::unity_engine::video::videotimereference::VideoTimeReference;

    #[method(name = "set_timeReference", args = 1)]
    pub fn set_time_reference(
        self,
        value: crate::unity_engine::video::videotimereference::VideoTimeReference,
    ) -> ();

    #[method(name = "get_externalReferenceTime", args = 0)]
    pub fn get_external_reference_time(self) -> f64;

    #[method(name = "set_externalReferenceTime", args = 1)]
    pub fn set_external_reference_time(self, value: f64) -> ();

    #[method(name = "get_canSetSkipOnDrop", args = 0)]
    pub fn get_can_set_skip_on_drop(self) -> bool;

    #[method(name = "get_skipOnDrop", args = 0)]
    pub fn get_skip_on_drop(self) -> bool;

    #[method(name = "set_skipOnDrop", args = 1)]
    pub fn set_skip_on_drop(self, value: bool) -> ();

    #[method(name = "get_frameCount", args = 0)]
    pub fn get_frame_count(self) -> u64;

    #[method(name = "get_frameRate", args = 0)]
    pub fn get_frame_rate(self) -> f32;

    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> f64;

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> u32;

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> u32;

    #[method(name = "get_pixelAspectRatioNumerator", args = 0)]
    pub fn get_pixel_aspect_ratio_numerator(self) -> u32;

    #[method(name = "get_pixelAspectRatioDenominator", args = 0)]
    pub fn get_pixel_aspect_ratio_denominator(self) -> u32;

    #[method(name = "get_audioTrackCount", args = 0)]
    pub fn get_audio_track_count(self) -> u16;

    #[method(name = "GetAudioLanguageCode", args = 1)]
    pub fn get_audio_language_code(self, track_index: u16) -> ::unity2::Il2CppString;

    #[method(name = "GetAudioChannelCount", args = 1)]
    pub fn get_audio_channel_count(self, track_index: u16) -> u16;

    #[method(name = "GetAudioSampleRate", args = 1)]
    pub fn get_audio_sample_rate(self, track_index: u16) -> u32;

    #[method(name = "get_controlledAudioTrackMaxCount", args = 0)]
    pub fn get_controlled_audio_track_max_count() -> u16;

    #[method(name = "get_controlledAudioTrackCount", args = 0)]
    pub fn get_controlled_audio_track_count(self) -> u16;

    #[method(name = "set_controlledAudioTrackCount", args = 1)]
    pub fn set_controlled_audio_track_count(self, value: u16) -> ();

    #[method(name = "EnableAudioTrack", args = 2)]
    pub fn enable_audio_track(self, track_index: u16, enabled: bool) -> ();

    #[method(name = "IsAudioTrackEnabled", args = 1)]
    pub fn is_audio_track_enabled(self, track_index: u16) -> bool;

    #[method(name = "get_audioOutputMode", args = 0)]
    pub fn get_audio_output_mode(
        self,
    ) -> crate::unity_engine::video::videoaudiooutputmode::VideoAudioOutputMode;

    #[method(name = "set_audioOutputMode", args = 1)]
    pub fn set_audio_output_mode(
        self,
        value: crate::unity_engine::video::videoaudiooutputmode::VideoAudioOutputMode,
    ) -> ();

    #[method(name = "get_canSetDirectAudioVolume", args = 0)]
    pub fn get_can_set_direct_audio_volume(self) -> bool;

    #[method(name = "GetDirectAudioVolume", args = 1)]
    pub fn get_direct_audio_volume(self, track_index: u16) -> f32;

    #[method(name = "SetDirectAudioVolume", args = 2)]
    pub fn set_direct_audio_volume(self, track_index: u16, volume: f32) -> ();

    #[method(name = "GetDirectAudioMute", args = 1)]
    pub fn get_direct_audio_mute(self, track_index: u16) -> bool;

    #[method(name = "SetDirectAudioMute", args = 2)]
    pub fn set_direct_audio_mute(self, track_index: u16, mute: bool) -> ();

    #[method(name = "GetTargetAudioSource", args = 1)]
    pub fn get_target_audio_source(
        self,
        track_index: u16,
    ) -> crate::unity_engine::audiosource::AudioSource;

    #[method(name = "SetTargetAudioSource", args = 2)]
    pub fn set_target_audio_source(
        self,
        track_index: u16,
        source: crate::unity_engine::audiosource::AudioSource,
    ) -> ();

    #[method(name = "add_prepareCompleted", args = 1)]
    pub fn add_prepare_completed(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "remove_prepareCompleted", args = 1)]
    pub fn remove_prepare_completed(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "add_loopPointReached", args = 1)]
    pub fn add_loop_point_reached(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "remove_loopPointReached", args = 1)]
    pub fn remove_loop_point_reached(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "add_started", args = 1)]
    pub fn add_started(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "remove_started", args = 1)]
    pub fn remove_started(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "add_frameDropped", args = 1)]
    pub fn add_frame_dropped(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "remove_frameDropped", args = 1)]
    pub fn remove_frame_dropped(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "add_errorReceived", args = 1)]
    pub fn add_error_received(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_ErrorEventHandler,
    ) -> ();

    #[method(name = "remove_errorReceived", args = 1)]
    pub fn remove_error_received(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_ErrorEventHandler,
    ) -> ();

    #[method(name = "add_seekCompleted", args = 1)]
    pub fn add_seek_completed(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "remove_seekCompleted", args = 1)]
    pub fn remove_seek_completed(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_EventHandler,
    ) -> ();

    #[method(name = "add_clockResyncOccurred", args = 1)]
    pub fn add_clock_resync_occurred(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_TimeEventHandler,
    ) -> ();

    #[method(name = "remove_clockResyncOccurred", args = 1)]
    pub fn remove_clock_resync_occurred(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_TimeEventHandler,
    ) -> ();

    #[method(name = "get_sendFrameReadyEvents", args = 0)]
    pub fn get_send_frame_ready_events(self) -> bool;

    #[method(name = "set_sendFrameReadyEvents", args = 1)]
    pub fn set_send_frame_ready_events(self, value: bool) -> ();

    #[method(name = "add_frameReady", args = 1)]
    pub fn add_frame_ready(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_FrameReadyEventHandler,
    ) -> ();

    #[method(name = "remove_frameReady", args = 1)]
    pub fn remove_frame_ready(
        self,
        value: crate::unity_engine::video::videoplayer::VideoPlayer_FrameReadyEventHandler,
    ) -> ();

    #[method(name = "InvokePrepareCompletedCallback_Internal", args = 1)]
    pub fn invoke_prepare_completed_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "InvokeFrameReadyCallback_Internal", args = 2)]
    pub fn invoke_frame_ready_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        frame_idx: i64,
    ) -> ();

    #[method(name = "InvokeLoopPointReachedCallback_Internal", args = 1)]
    pub fn invoke_loop_point_reached_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "InvokeStartedCallback_Internal", args = 1)]
    pub fn invoke_started_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "InvokeFrameDroppedCallback_Internal", args = 1)]
    pub fn invoke_frame_dropped_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "InvokeErrorReceivedCallback_Internal", args = 2)]
    pub fn invoke_error_received_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        error_str: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "InvokeSeekCompletedCallback_Internal", args = 1)]
    pub fn invoke_seek_completed_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
    ) -> ();

    #[method(name = "InvokeClockResyncOccurredCallback_Internal", args = 2)]
    pub fn invoke_clock_resync_occurred_callback_internal(
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        seconds: f64,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-video-videoplayer")]
impl VideoPlayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VideoPlayer),
                ::core::stringify!(new),
            )
        });
        <Self as IVideoPlayerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/video/videoplayer/VideoPlayer_ErrorEventHandler.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Video",
    name = "VideoPlayer.ErrorEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VideoPlayer_ErrorEventHandler {}

#[cfg(feature = "unity_engine-video-videoplayer")]
#[::unity2::methods]
impl VideoPlayer_ErrorEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        message: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "unity_engine-video-videoplayer")]
impl VideoPlayer_ErrorEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VideoPlayer_ErrorEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IVideoPlayer_ErrorEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/video/videoplayer/VideoPlayer_EventHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.Video", name = "VideoPlayer.EventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VideoPlayer_EventHandler {}

#[cfg(feature = "unity_engine-video-videoplayer")]
#[::unity2::methods]
impl VideoPlayer_EventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, source: crate::unity_engine::video::videoplayer::VideoPlayer) -> ();
}

#[cfg(feature = "unity_engine-video-videoplayer")]
impl VideoPlayer_EventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VideoPlayer_EventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IVideoPlayer_EventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/video/videoplayer/VideoPlayer_TimeEventHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.Video", name = "VideoPlayer.TimeEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VideoPlayer_TimeEventHandler {}

#[cfg(feature = "unity_engine-video-videoplayer")]
#[::unity2::methods]
impl VideoPlayer_TimeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        source: crate::unity_engine::video::videoplayer::VideoPlayer,
        seconds: f64,
    ) -> ();
}

#[cfg(feature = "unity_engine-video-videoplayer")]
impl VideoPlayer_TimeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VideoPlayer_TimeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IVideoPlayer_TimeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
