
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::app::singletonpool_2::ISingletonPool_2;
use crate::app::singletonpool_2::SingletonPool_2;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godpool/GodPool.md")))]
#[::unity2::class(namespace = "App", name = "GodPool")]
# [parent (crate :: app :: singletonpool_2 :: SingletonPool_2 < crate :: app :: godpool :: GodPool , crate :: app :: godunit :: GodUnit >)]
pub struct GodPool {}

#[cfg(feature = "app-godpool")]
#[::unity2::methods]
impl GodPool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(
        gid: ::unity2::Il2CppString,
        include_reserved: bool,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_2(
        data: crate::app::goddata::GodData,
        include_reserved: bool,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "TryGetImpl", args = 2)]
    pub fn try_get_impl(
        data: crate::app::goddata::GodData,
        include_reserved: bool,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "Create", args = 1)]
    pub fn create(data: crate::app::goddata::GodData) -> crate::app::godunit::GodUnit;

    #[method(name = "Delete", args = 1)]
    pub fn delete(god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "DeleteOrReserve", args = 1)]
    pub fn delete_or_reserve(god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "DeleteReserved", args = 0)]
    pub fn delete_reserved() -> ();

    #[method(name = "DeleteExceptForPlayer", args = 0)]
    pub fn delete_except_for_player() -> ();

    #[method(name = "AppendEnemyGod", args = 1)]
    pub fn append_enemy_god(god: crate::app::godunit::GodUnit) -> crate::app::godunit::GodUnit;

    #[method(name = "HasArmlet", args = 0)]
    pub fn has_armlet() -> bool;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-godpool")]
impl GodPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodPool),
                ::core::stringify!(new),
            )
        });
        <Self as IGodPoolMethods>::ctor(this);
        this
    }
}
