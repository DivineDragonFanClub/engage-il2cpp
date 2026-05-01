
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkbuilder/TalkBuilder.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkBuilder")]
#[parent(crate::system::object::Object)]
pub struct TalkBuilder {
    #[static_field]
    #[rename(name = "BufSize")]
    pub buf_size: i32,
    #[rename(name = "m_TalkPtr")]
    pub m_talk_ptr: crate::app::talk3_d::talkptr::TalkPtr,
    #[rename(name = "m_IntPtr")]
    pub m_int_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "app-talk3_d-talkbuilder")]
#[::unity2::methods]
impl TalkBuilder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "AddTalkType", args = 2)]
    pub fn add_talk_type(
        self,
        r#type: crate::app::talk3_d::talk_2::Talk_TalkType,
        positions_root_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddKeyWait", args = 0)]
    pub fn add_key_wait(self) -> ();

    #[method(name = "AddKeyNextPage", args = 0)]
    pub fn add_key_next_page(self) -> ();

    #[method(name = "AddKeyWaitAndNextPage", args = 0)]
    pub fn add_key_wait_and_next_page(self) -> ();

    #[method(name = "AddWindowMake", args = 2)]
    pub fn add_window_make(
        self,
        pid: ::unity2::Il2CppString,
        location: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddWindowDelete", args = 1)]
    pub fn add_window_delete(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "AddWindowActive", args = 1)]
    pub fn add_window_active(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "AddString", args = 1)]
    pub fn add_string(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "Terminate", args = 0)]
    pub fn terminate(self) -> ();

    #[method(name = "GetPtr", args = 0)]
    pub fn get_ptr(self) -> ::unity2::IntPtr;
}

#[cfg(feature = "app-talk3_d-talkbuilder")]
impl TalkBuilder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkBuilder),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkBuilderMethods>::ctor(this);
        this
    }
}
