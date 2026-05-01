
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayreplaycache/RelayReplayCache.md")))]
#[::unity2::class(namespace = "App", name = "RelayReplayCache")]
#[parent(crate::system::object::Object)]
pub struct RelayReplayCache {
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
    #[rename(name = "m_IsNoNeed")]
    pub m_is_no_need: bool,
    #[rename(name = "m_Data")]
    pub m_data: crate::system::collections::generic::list_1::List_1<
        crate::app::relayservermetadata::RelayServerMetaData,
    >,
}

#[cfg(feature = "app-relayreplaycache")]
#[::unity2::methods]
impl RelayReplayCache {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        self,
        search_results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "CopyTo", args = 1)]
    pub fn copy_to(
        self,
        search_results: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "Duplicate", args = 2)]
    pub fn duplicate(
        self,
        src: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
        dst: crate::system::collections::generic::list_1::List_1<
            crate::app::relayservermetadata::RelayServerMetaData,
        >,
    ) -> ();

    #[method(name = "Replace", args = 1)]
    pub fn replace(self, new_meta_data: crate::app::relayservermetadata::RelayServerMetaData)
        -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_IsValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "get_IsNoNeed", args = 0)]
    pub fn get_is_no_need(self) -> bool;

    #[method(name = "set_IsNoNeed", args = 1)]
    pub fn set_is_no_need(self, value: bool) -> ();
}

#[cfg(feature = "app-relayreplaycache")]
impl RelayReplayCache {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayReplayCache),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayReplayCacheMethods>::ctor(this);
        this
    }
}
