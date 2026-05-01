
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/vibrationmanager/VibrationManager_VibHandle.md")))]
#[::unity2::class(namespace = "App", name = "VibrationManager.VibHandle")]
#[parent(crate::system::object::Object)]
pub struct VibrationManager_VibHandle {
    #[static_field]
    #[rename(name = "DEVICE_COUNT_MAX")]
    pub device_count_max: i32,
    #[rename(name = "m_deviceCount")]
    pub m_device_count: i32,
    #[rename(name = "m_amplitudeMagnitude")]
    pub m_amplitude_magnitude: f32,
    #[rename(name = "m_file")]
    pub m_file: crate::app::vibrationfile::VibrationFile,
    #[rename(name = "m_sample")]
    pub m_sample: i32,
    #[rename(name = "m_sampleLoop")]
    pub m_sample_loop: bool,
}

#[cfg(feature = "app-vibrationmanager")]
#[::unity2::methods]
impl VibrationManager_VibHandle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, file: crate::app::vibrationfile::VibrationFile, is_loop: bool) -> ();

    #[method(name = "SetAmplitudeMagnitude", args = 1)]
    pub fn set_amplitude_magnitude(self, amplitude_magnitude: f32) -> ();

    #[method(name = "SetAmplitude", args = 2)]
    pub fn set_amplitude(self, amp_low: f32, amp_high: f32) -> ();

    #[method(name = "SetFrequecy", args = 2)]
    pub fn set_frequecy(self, freq_low: f32, freq_high: f32) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();
}

#[cfg(feature = "app-vibrationmanager")]
impl VibrationManager_VibHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VibrationManager_VibHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IVibrationManager_VibHandleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/vibrationmanager/VibrationManager.md")))]
#[::unity2::class(namespace = "App", name = "VibrationManager")]
#[parent(crate::system::object::Object)]
pub struct VibrationManager {
    #[rename(name = "m_vibrationFileDictionary")]
    pub m_vibration_file_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::vibrationfile::VibrationFile,
        >,
    #[rename(name = "m_handle")]
    pub m_handle: crate::app::vibrationmanager::VibrationManager_VibHandle,
}

#[cfg(feature = "app-vibrationmanager")]
#[::unity2::methods]
impl VibrationManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetFile", args = 2)]
    pub fn set_file(self, file: crate::app::vibrationfile::VibrationFile, is_loop: bool) -> ();

    #[method(name = "SetAmplitudeMagnitude", args = 1)]
    pub fn set_amplitude_magnitude(self, amplitude_magnitude: f32) -> ();

    #[method(name = "SetAmplitude", args = 2)]
    pub fn set_amplitude(self, amp_low: f32, amp_high: f32) -> ();

    #[method(name = "SetFrequecy", args = 2)]
    pub fn set_frequecy(self, freq_low: f32, freq_high: f32) -> ();

    #[method(name = "OneShot", args = 4)]
    pub fn one_shot(self, time: f32, amplitude_magnitude: f32, amp_low: f32, amp_high: f32) -> ();

    #[method(name = "OneShot", args = 6)]
    pub fn one_shot_2(
        self,
        time: f32,
        amplitude_magnitude: f32,
        amp_low: f32,
        amp_high: f32,
        freq_low: f32,
        freq_high: f32,
    ) -> ();

    #[method(name = "StopVibe", args = 0)]
    pub fn stop_vibe(self) -> ();

    #[method(name = "PlayByGameSoundEvent", args = 3)]
    pub fn play_by_game_sound_event(
        self,
        event_name: ::unity2::Il2CppString,
        time: f32,
        is_loop: bool,
    ) -> ();

    #[method(name = "PlayByVibrationFileName", args = 4)]
    pub fn play_by_vibration_file_name(
        self,
        vib_file_name: ::unity2::Il2CppString,
        amplitude_magnitude: f32,
        time: f32,
        is_loop: bool,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();
}

#[cfg(feature = "app-vibrationmanager")]
impl VibrationManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VibrationManager),
                ::core::stringify!(new),
            )
        });
        <Self as IVibrationManagerMethods>::ctor(this);
        this
    }
}
