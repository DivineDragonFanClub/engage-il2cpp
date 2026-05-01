
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayunsettledsequence/RelayUnsettledSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayUnsettledSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RelayUnsettledSequence {}

#[cfg(feature = "app-relayunsettledsequence")]
#[::unity2::methods]
impl RelayUnsettledSequence {
    #[method(name = "NotifyPlayEnd", args = 0)]
    pub fn notify_play_end(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayunsettledsequence")]
impl RelayUnsettledSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayUnsettledSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayUnsettledSequenceMethods>::ctor(this);
        this
    }
}
