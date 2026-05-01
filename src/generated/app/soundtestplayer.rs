
use crate::app::bgmplayer::BgmPlayer;
use crate::app::bgmplayer::IBgmPlayer;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundtestplayer/SoundTestPlayer.md")))]
#[::unity2::class(namespace = "App", name = "SoundTestPlayer")]
#[parent(crate::app::bgmplayer::BgmPlayer)]
pub struct SoundTestPlayer {
    #[rename(name = "m_handle")]
    pub m_handle: crate::app::gamesound::GameSound_Handle,
    #[rename(name = "m_currentPause")]
    pub m_current_pause: bool,
    #[rename(name = "m_soundList")]
    pub m_sound_list: crate::system::collections::generic::list_1::List_1<
        crate::app::gamesound::GameSound_Handle,
    >,
}

#[cfg(feature = "app-soundtestplayer")]
#[::unity2::methods]
impl SoundTestPlayer {
    #[method(name = "PauseCurrentBgm", args = 0)]
    pub fn pause_current_bgm(self) -> ();

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "IsPlaying", args = 1)]
    pub fn is_playing_2(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Play", args = 1)]
    pub fn play(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Tick", args = 1)]
    pub fn tick(self, menu: crate::app::myroomsoundmenu::MyRoomSoundMenu) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundtestplayer")]
impl SoundTestPlayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundTestPlayer),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundTestPlayerMethods>::ctor(this);
        this
    }
}
