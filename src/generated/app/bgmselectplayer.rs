
use crate::app::bgmplayer::BgmPlayer;
use crate::app::bgmplayer::IBgmPlayer;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bgmselectplayer/BgmSelectPlayer.md")))]
#[::unity2::class(namespace = "App", name = "BgmSelectPlayer")]
#[parent(crate::app::bgmplayer::BgmPlayer)]
pub struct BgmSelectPlayer {
    #[rename(name = "m_handle")]
    pub m_handle: crate::app::gamesound::GameSound_Handle,
    #[rename(name = "m_soundList")]
    pub m_sound_list: crate::system::collections::generic::list_1::List_1<
        crate::app::gamesound::GameSound_Handle,
    >,
}

#[cfg(feature = "app-bgmselectplayer")]
#[::unity2::methods]
impl BgmSelectPlayer {
    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "IsPlaying", args = 1)]
    pub fn is_playing(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PauseCurrentBgm", args = 0)]
    pub fn pause_current_bgm(self) -> ();

    #[method(name = "PlaySelect", args = 1)]
    pub fn play_select(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-bgmselectplayer")]
impl BgmSelectPlayer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BgmSelectPlayer),
                ::core::stringify!(new),
            )
        });
        <Self as IBgmSelectPlayerMethods>::ctor(this);
        this
    }
}
