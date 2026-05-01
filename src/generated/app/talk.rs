
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk/Talk.md")))]
#[::unity2::class(namespace = "App", name = "Talk")]
#[parent(crate::system::object::Object)]
pub struct Talk {}

#[cfg(feature = "app-talk")]
#[::unity2::methods]
impl Talk {
    #[method(name = "StartBind", args = 3)]
    pub fn start_bind(
        parent: crate::app::procinst::ProcInst,
        mid: ::unity2::Il2CppString,
        is_continuous_number: bool,
    ) -> ();

    #[method(name = "BeginContinue", args = 0)]
    pub fn begin_continue() -> ();

    #[method(name = "EndContinue", args = 0)]
    pub fn end_continue() -> ();

    #[method(name = "IsFastForward", args = 0)]
    pub fn is_fast_forward() -> bool;

    #[method(name = "IsPageTrigger", args = 0)]
    pub fn is_page_trigger() -> bool;

    #[method(name = "IsAutoPlayMode", args = 0)]
    pub fn is_auto_play_mode() -> bool;

    #[method(name = "SetPlayMode", args = 1)]
    pub fn set_play_mode(
        talk_play_mode: crate::app::talk3_d::talksequence::TalkSequence_PlayMode,
    ) -> ();

    #[method(name = "GetPlayingMid", args = 0)]
    pub fn get_playing_mid() -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk")]
impl Talk {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Talk),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkMethods>::ctor(this);
        this
    }
}
