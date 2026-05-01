
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsoundmenuitem/MyRoomSoundMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSoundMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomSoundMenuItem {
    #[rename(name = "m_eventName")]
    pub m_event_name: ::unity2::Il2CppString,
    #[rename(name = "m_label")]
    pub m_label: ::unity2::Il2CppString,
    #[rename(name = "m_help")]
    pub m_help: ::unity2::Il2CppString,
}

#[cfg(feature = "app-myroomsoundmenuitem")]
#[::unity2::methods]
impl MyRoomSoundMenuItem {
    #[method(name = "get_Callback", args = 0)]
    pub fn get_callback(self) -> crate::system::action_1::Action_1<::unity2::Il2CppString>;

    #[method(name = "get_IsPlay", args = 0)]
    pub fn get_is_play(self) -> bool;

    #[method(name = "get_EventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        event_name: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        help: ::unity2::Il2CppString,
        callback: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "SetCurrentMusicName", args = 0)]
    pub fn set_current_music_name(self) -> ();

    #[method(name = "SetMusicName", args = 1)]
    pub fn set_music_name(self, music_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetMusicHelpText", args = 1)]
    pub fn set_music_help_text(self, text: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-myroomsoundmenuitem")]
impl MyRoomSoundMenuItem {
    pub fn new(
        event_name: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        help: ::unity2::Il2CppString,
        callback: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSoundMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSoundMenuItemMethods>::ctor(this, event_name, name, help, callback);
        this
    }
}
