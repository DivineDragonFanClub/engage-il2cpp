
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procwaitmessagebase/ProcWaitMessageBase.md")))]
#[::unity2::class(namespace = "App", name = "ProcWaitMessageBase")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProcWaitMessageBase {
    #[rename(name = "m_GameMessage")]
    pub m_game_message: crate::app::gamemessage::GameMessage,
}

#[cfg(feature = "app-procwaitmessagebase")]
#[::unity2::methods]
impl ProcWaitMessageBase {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "MessageOpenKeyWait", args = 1)]
    pub fn message_open_key_wait(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "MessageOpenSystemWait", args = 2)]
    pub fn message_open_system_wait(
        self,
        msg: ::unity2::Il2CppString,
        is_hide_wait_anime: bool,
    ) -> ();

    #[method(name = "MessageClose", args = 0)]
    pub fn message_close(self) -> ();

    #[method(name = "MessageDelete", args = 0)]
    pub fn message_delete(self) -> ();

    #[method(name = "MessageIsWaitToOpen", args = 0)]
    pub fn message_is_wait_to_open(self) -> bool;

    #[method(name = "MessageIsWaitToClose", args = 0)]
    pub fn message_is_wait_to_close(self) -> bool;
}

#[cfg(feature = "app-procwaitmessagebase")]
impl ProcWaitMessageBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcWaitMessageBase),
                ::core::stringify!(new),
            )
        });
        <Self as IProcWaitMessageBaseMethods>::ctor(this);
        this
    }
}
