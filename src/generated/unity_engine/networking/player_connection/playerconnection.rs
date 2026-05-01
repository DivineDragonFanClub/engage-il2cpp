
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/player_connection/playerconnection/PlayerConnection.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking.PlayerConnection",
    name = "PlayerConnection"
)]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct PlayerConnection {
# [static_field] # [rename (name = "connectionNative")] pub connection_native : crate :: unity_engine :: iplayereditorconnectionnative :: IPlayerEditorConnectionNative ,
# [rename (name = "m_PlayerEditorConnectionEvents")] pub m_player_editor_connection_events : crate :: unity_engine :: networking :: player_connection :: playereditorconnectionevents :: PlayerEditorConnectionEvents ,
# [rename (name = "m_connectedPlayers")] pub m_connected_players : crate :: system :: collections :: generic :: list_1 :: List_1 < i32 > ,
# [rename (name = "m_IsInitilized")] pub m_is_initilized : bool ,
# [static_field] # [rename (name = "s_Instance")] pub s_instance : crate :: unity_engine :: networking :: player_connection :: playerconnection :: PlayerConnection ,
}

#[cfg(feature = "unity_engine-networking-player_connection-playerconnection")]
#[::unity2::methods]
impl PlayerConnection {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance(
    ) -> crate::unity_engine::networking::player_connection::playerconnection::PlayerConnection;

    #[method(name = "get_isConnected", args = 0)]
    pub fn get_is_connected(self) -> bool;

    #[method(name = "CreateInstance", args = 0)]
    pub fn create_instance(
    ) -> crate::unity_engine::networking::player_connection::playerconnection::PlayerConnection;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "GetConnectionNativeApi", args = 0)]
    pub fn get_connection_native_api(
        self,
    ) -> crate::unity_engine::iplayereditorconnectionnative::IPlayerEditorConnectionNative;

    #[method(name = "RegisterConnection", args = 1)]
    pub fn register_connection(
        self,
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<i32>,
    ) -> ();

    #[method(name = "RegisterDisconnection", args = 1)]
    pub fn register_disconnection(
        self,
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<i32>,
    ) -> ();

    #[method(name = "UnregisterConnection", args = 1)]
    pub fn unregister_connection(
        self,
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<i32>,
    ) -> ();

    #[method(name = "UnregisterDisconnection", args = 1)]
    pub fn unregister_disconnection(
        self,
        callback: crate::unity_engine::events::unityaction_1::UnityAction_1<i32>,
    ) -> ();

    #[method(name = "DisconnectAll", args = 0)]
    pub fn disconnect_all(self) -> ();

    #[method(name = "MessageCallbackInternal", args = 4)]
    pub fn message_callback_internal(
        data: ::unity2::IntPtr,
        size: u64,
        guid: u64,
        message_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ConnectedCallbackInternal", args = 1)]
    pub fn connected_callback_internal(player_id: i32) -> ();

    #[method(name = "DisconnectedCallback", args = 1)]
    pub fn disconnected_callback(player_id: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-player_connection-playerconnection")]
impl PlayerConnection {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerConnection),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerConnectionMethods>::ctor(this);
        this
    }
}
