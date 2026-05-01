
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/low_level/playerloop/PlayerLoop.md")))]
#[::unity2::class(namespace = "UnityEngine.LowLevel", name = "PlayerLoop")]
#[parent(crate::system::object::Object)]
pub struct PlayerLoop {}

#[cfg(feature = "unity_engine-low_level-playerloop")]
#[::unity2::methods]
impl PlayerLoop {
    #[method(name = "GetDefaultPlayerLoop", args = 0)]
    pub fn get_default_player_loop(
    ) -> crate::unity_engine::low_level::playerloopsystem::PlayerLoopSystem;

    #[method(name = "SetPlayerLoop", args = 1)]
    pub fn set_player_loop(
        r#loop: crate::unity_engine::low_level::playerloopsystem::PlayerLoopSystem,
    ) -> ();

    #[method(name = "PlayerLoopSystemToInternal", args = 2)]
    pub fn player_loop_system_to_internal(
        sys: crate::unity_engine::low_level::playerloopsystem::PlayerLoopSystem,
        internal_sys: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::low_level::playerloopsysteminternal::PlayerLoopSystemInternal,
        >,
    ) -> i32;

    #[method(name = "InternalToPlayerLoopSystem", args = 2)]
    pub fn internal_to_player_loop_system(
        internal_sys: ::unity2::Array<
            crate::unity_engine::low_level::playerloopsysteminternal::PlayerLoopSystemInternal,
        >,
        offset: i32,
    ) -> crate::unity_engine::low_level::playerloopsystem::PlayerLoopSystem;

    #[method(name = "GetDefaultPlayerLoopInternal", args = 0)]
    pub fn get_default_player_loop_internal() -> ::unity2::Array<
        crate::unity_engine::low_level::playerloopsysteminternal::PlayerLoopSystemInternal,
    >;

    #[method(name = "SetPlayerLoopInternal", args = 1)]
    pub fn set_player_loop_internal(
        r#loop: ::unity2::Array<
            crate::unity_engine::low_level::playerloopsysteminternal::PlayerLoopSystemInternal,
        >,
    ) -> ();
}
