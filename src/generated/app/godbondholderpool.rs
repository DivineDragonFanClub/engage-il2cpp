
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::app::singletonpool_2::ISingletonPool_2;
use crate::app::singletonpool_2::SingletonPool_2;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godbondholderpool/GodBondHolderPool.md")))]
#[::unity2::class(namespace = "App", name = "GodBondHolderPool")]
# [parent (crate :: app :: singletonpool_2 :: SingletonPool_2 < crate :: app :: godbondholderpool :: GodBondHolderPool , crate :: app :: godbondholder :: GodBondHolder >)]
pub struct GodBondHolderPool {}

#[cfg(feature = "app-godbondholderpool")]
#[::unity2::methods]
impl GodBondHolderPool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "CreateOrGet", args = 1)]
    pub fn create_or_get(
        self,
        data: crate::app::goddata::GodData,
    ) -> crate::app::godbondholder::GodBondHolder;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "SerializeForRewindLatest", args = 2)]
    pub fn serialize_for_rewind_latest(
        self,
        stream: crate::app::stream_2::Stream_2,
        exclude_pids: crate::system::collections::generic::hashset_1::HashSet_1<
            ::unity2::Il2CppString,
        >,
    ) -> ();

    #[method(name = "DeserializeForRewindLatest", args = 1)]
    pub fn deserialize_for_rewind_latest(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-godbondholderpool")]
impl GodBondHolderPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodBondHolderPool),
                ::core::stringify!(new),
            )
        });
        <Self as IGodBondHolderPoolMethods>::ctor(this);
        this
    }
}
