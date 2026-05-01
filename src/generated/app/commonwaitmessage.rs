
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procwaitmessagebase::IProcWaitMessageBase;
use crate::app::procwaitmessagebase::ProcWaitMessageBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonwaitmessage/CommonWaitMessage.md")))]
#[::unity2::class(namespace = "App", name = "CommonWaitMessage")]
#[parent(crate::app::procwaitmessagebase::ProcWaitMessageBase)]
pub struct CommonWaitMessage {
    #[rename(name = "m_IsLoopSE")]
    pub m_is_loop_se: bool,
    #[rename(name = "m_IsPlaySE")]
    pub m_is_play_se: bool,
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::app::commonwaitmessage::CommonWaitMessage,
}

#[cfg(feature = "app-commonwaitmessage")]
#[::unity2::methods]
impl CommonWaitMessage {
    #[method(name = "Open", args = 3)]
    pub fn open(msg: ::unity2::Il2CppString, is_play_sound: bool, is_hide_wait_anime: bool) -> ();

    #[method(name = "Close", args = 1)]
    pub fn close(is_success: bool) -> ();

    #[method(name = "IsAlive", args = 0)]
    pub fn is_alive() -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_play_sound: bool) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "PlayLoopSE", args = 0)]
    pub fn play_loop_se(self) -> ();

    #[method(name = "StopLoopSE", args = 0)]
    pub fn stop_loop_se(self) -> ();

    #[method(name = "PlayResultSE", args = 1)]
    pub fn play_result_se(self, is_success: bool) -> ();
}

#[cfg(feature = "app-commonwaitmessage")]
impl CommonWaitMessage {
    pub fn new(is_play_sound: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonWaitMessage),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonWaitMessageMethods>::ctor(this, is_play_sound);
        this
    }
}
