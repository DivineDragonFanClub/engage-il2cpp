
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionsync/ActionSync.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionSync")]
#[parent(crate::combat::actionbase::ActionBase)]
pub struct ActionSync {
    #[rename(name = "m_SyncToken")]
    pub m_sync_token: crate::combat::synctoken::SyncToken,
}

#[cfg(feature = "combat-actionsync")]
#[::unity2::methods]
impl ActionSync {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        sync_token: crate::combat::synctoken::SyncToken,
    ) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionsync")]
impl ActionSync {
    pub fn new(
        chr: crate::combat::character::Character,
        sync_token: crate::combat::synctoken::SyncToken,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionSync),
                ::core::stringify!(new),
            )
        });
        <Self as IActionSyncMethods>::ctor(this, chr, sync_token);
        this
    }
}
