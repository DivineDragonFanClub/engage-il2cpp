
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playerconnectioninternal/PlayerConnectionInternal.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "PlayerConnectionInternal")]
#[parent(crate::system::object::Object)]
pub struct PlayerConnectionInternal {}

#[cfg(feature = "unity_engine-playerconnectioninternal")]
#[::unity2::methods]
impl PlayerConnectionInternal {
    #[method(name = "UnityEngine.IPlayerEditorConnectionNative.Poll", args = 0)]
    pub fn unity_engine_i_player_editor_connection_native_poll(self) -> ();

    #[method(
        name = "UnityEngine.IPlayerEditorConnectionNative.Initialize",
        args = 0
    )]
    pub fn unity_engine_i_player_editor_connection_native_initialize(self) -> ();

    #[method(
        name = "UnityEngine.IPlayerEditorConnectionNative.IsConnected",
        args = 0
    )]
    pub fn unity_engine_i_player_editor_connection_native_is_connected(self) -> bool;

    #[method(
        name = "UnityEngine.IPlayerEditorConnectionNative.DisconnectAll",
        args = 0
    )]
    pub fn unity_engine_i_player_editor_connection_native_disconnect_all(self) -> ();

    #[method(name = "IsConnected", args = 0)]
    pub fn is_connected() -> bool;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "RegisterInternal", args = 1)]
    pub fn register_internal(message_id: ::unity2::Il2CppString) -> ();

    #[method(name = "UnregisterInternal", args = 1)]
    pub fn unregister_internal(message_id: ::unity2::Il2CppString) -> ();

    #[method(name = "SendMessage", args = 3)]
    pub fn send_message(
        message_id: ::unity2::Il2CppString,
        data: ::unity2::Array<u8>,
        player_id: i32,
    ) -> ();

    #[method(name = "TrySendMessage", args = 3)]
    pub fn try_send_message(
        message_id: ::unity2::Il2CppString,
        data: ::unity2::Array<u8>,
        player_id: i32,
    ) -> bool;

    #[method(name = "PollInternal", args = 0)]
    pub fn poll_internal() -> ();

    #[method(name = "DisconnectAll", args = 0)]
    pub fn disconnect_all() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-playerconnectioninternal")]
impl PlayerConnectionInternal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerConnectionInternal),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerConnectionInternalMethods>::ctor(this);
        this
    }
}
