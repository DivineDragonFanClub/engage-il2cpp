
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmovestate/HubMoveState.md")))]
#[::unity2::class(namespace = "App", name = "HubMoveState")]
#[parent(crate::system::object::Object)]
pub struct HubMoveState {}

#[cfg(feature = "app-hubmovestate")]
#[::unity2::methods]
impl HubMoveState {
    #[method(name = "IsEnd", args = 0)]
    pub fn is_end(self) -> bool;

    #[method(name = "Start", args = 1)]
    pub fn start(self, resume: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmovestate")]
impl HubMoveState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMoveState),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMoveStateMethods>::ctor(this);
        this
    }
}
