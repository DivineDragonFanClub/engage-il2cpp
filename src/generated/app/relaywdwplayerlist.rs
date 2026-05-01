
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaywdwplayerlist/RelayWdwPlayerList.md")))]
#[::unity2::class(namespace = "App", name = "RelayWdwPlayerList")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RelayWdwPlayerList {
    #[rename(name = "m_PlayerList")]
    pub m_player_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-relaywdwplayerlist")]
#[::unity2::methods]
impl RelayWdwPlayerList {
    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, meta_data: crate::app::relayservermetadata::RelayServerMetaData) -> ();

    #[method(name = "SetLastPlayer", args = 2)]
    pub fn set_last_player(self, idx: i32, player_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaywdwplayerlist")]
impl RelayWdwPlayerList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayWdwPlayerList),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayWdwPlayerListMethods>::ctor(this);
        this
    }
}
