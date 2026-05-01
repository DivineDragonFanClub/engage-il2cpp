
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/player_connection/playereditorconnectionevents/PlayerEditorConnectionEvents_ConnectionChangeEvent.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking.PlayerConnection",
    name = "PlayerEditorConnectionEvents.ConnectionChangeEvent"
)]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < i32 >)]
pub struct PlayerEditorConnectionEvents_ConnectionChangeEvent {}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
#[::unity2::methods]
impl PlayerEditorConnectionEvents_ConnectionChangeEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
impl PlayerEditorConnectionEvents_ConnectionChangeEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerEditorConnectionEvents_ConnectionChangeEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerEditorConnectionEvents_ConnectionChangeEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/player_connection/playereditorconnectionevents/PlayerEditorConnectionEvents.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking.PlayerConnection",
    name = "PlayerEditorConnectionEvents"
)]
#[parent(crate::system::object::Object)]
pub struct PlayerEditorConnectionEvents {
# [rename (name = "messageTypeSubscribers")] pub message_type_subscribers : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: networking :: player_connection :: playereditorconnectionevents :: PlayerEditorConnectionEvents_MessageTypeSubscribers > ,
# [rename (name = "connectionEvent")] pub connection_event : crate :: unity_engine :: networking :: player_connection :: playereditorconnectionevents :: PlayerEditorConnectionEvents_ConnectionChangeEvent ,
# [rename (name = "disconnectionEvent")] pub disconnection_event : crate :: unity_engine :: networking :: player_connection :: playereditorconnectionevents :: PlayerEditorConnectionEvents_ConnectionChangeEvent ,
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
#[::unity2::methods]
impl PlayerEditorConnectionEvents {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
impl PlayerEditorConnectionEvents {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerEditorConnectionEvents),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerEditorConnectionEventsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/player_connection/playereditorconnectionevents/PlayerEditorConnectionEvents_MessageEvent.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking.PlayerConnection",
    name = "PlayerEditorConnectionEvents.MessageEvent"
)]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < crate :: unity_engine :: networking :: player_connection :: messageeventargs :: MessageEventArgs >)]
pub struct PlayerEditorConnectionEvents_MessageEvent {}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
#[::unity2::methods]
impl PlayerEditorConnectionEvents_MessageEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
impl PlayerEditorConnectionEvents_MessageEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerEditorConnectionEvents_MessageEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerEditorConnectionEvents_MessageEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/player_connection/playereditorconnectionevents/PlayerEditorConnectionEvents_MessageTypeSubscribers.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking.PlayerConnection",
    name = "PlayerEditorConnectionEvents.MessageTypeSubscribers"
)]
#[parent(crate::system::object::Object)]
pub struct PlayerEditorConnectionEvents_MessageTypeSubscribers {
# [rename (name = "m_messageTypeId")] pub m_message_type_id : :: unity2 :: Il2CppString ,
# [rename (name = "subscriberCount")] pub subscriber_count : i32 ,
# [rename (name = "messageCallback")] pub message_callback : crate :: unity_engine :: networking :: player_connection :: playereditorconnectionevents :: PlayerEditorConnectionEvents_MessageEvent ,
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
#[::unity2::methods]
impl PlayerEditorConnectionEvents_MessageTypeSubscribers {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-player_connection-playereditorconnectionevents")]
impl PlayerEditorConnectionEvents_MessageTypeSubscribers {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerEditorConnectionEvents_MessageTypeSubscribers),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerEditorConnectionEvents_MessageTypeSubscribersMethods>::ctor(this);
        this
    }
}
