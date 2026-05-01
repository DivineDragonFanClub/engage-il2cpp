
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserdata/RelayUserData_EnteredBattle.md")))]
#[::unity2::class(namespace = "App", name = "RelayUserData.EnteredBattle")]
#[parent(crate::system::object::Object)]
pub struct RelayUserData_EnteredBattle {}

#[cfg(feature = "app-relayuserdata")]
#[::unity2::methods]
impl RelayUserData_EnteredBattle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_DataId", args = 0)]
    pub fn get_data_id(self) -> u64;

    #[method(name = "set_DataId", args = 1)]
    pub fn set_data_id(self, value: u64) -> ();

    #[method(name = "get_PlayerIndex", args = 0)]
    pub fn get_player_index(self) -> i32;

    #[method(name = "set_PlayerIndex", args = 1)]
    pub fn set_player_index(self, value: i32) -> ();

    #[method(name = "get_IsAwarded", args = 0)]
    pub fn get_is_awarded(self) -> bool;

    #[method(name = "set_IsAwarded", args = 1)]
    pub fn set_is_awarded(self, value: bool) -> ();
}

#[cfg(feature = "app-relayuserdata")]
impl RelayUserData_EnteredBattle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserData_EnteredBattle),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUserData_EnteredBattleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayuserdata/RelayUserData.md")))]
#[::unity2::class(namespace = "App", name = "RelayUserData")]
#[parent(crate::system::object::Object)]
pub struct RelayUserData {
    #[static_field]
    #[rename(name = "MaxEnteredBattle")]
    pub max_entered_battle: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: u8,
    #[rename(name = "m_EnteredBattles")]
    pub m_entered_battles: crate::system::collections::generic::list_1::List_1<
        crate::app::relayuserdata::RelayUserData_EnteredBattle,
    >,
}

#[cfg(feature = "app-relayuserdata")]
#[::unity2::methods]
impl RelayUserData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, from: crate::app::relayuserdata::RelayUserData) -> ();

    #[method(name = "AddEnteredBattle", args = 1)]
    pub fn add_entered_battle(
        self,
        new_battle: crate::app::relayuserdata::RelayUserData_EnteredBattle,
    ) -> ();

    #[method(name = "AddEnteredBattleImpl", args = 1)]
    pub fn add_entered_battle_impl(
        self,
        new_battle: crate::app::relayuserdata::RelayUserData_EnteredBattle,
    ) -> bool;

    #[method(name = "GetEnteredBattle", args = 1)]
    pub fn get_entered_battle(
        self,
        data_id: u64,
    ) -> crate::app::relayuserdata::RelayUserData_EnteredBattle;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_EnteredBattles", args = 0)]
    pub fn get_entered_battles(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::relayuserdata::RelayUserData_EnteredBattle,
    >;

    #[method(name = "DbgDump", args = 0)]
    pub fn dbg_dump(self) -> ();
}

#[cfg(feature = "app-relayuserdata")]
impl RelayUserData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUserDataMethods>::ctor(this);
        this
    }

    pub fn new_2(from: crate::app::relayuserdata::RelayUserData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUserData),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRelayUserDataMethods>::ctor_2(this, from);
        this
    }
}
