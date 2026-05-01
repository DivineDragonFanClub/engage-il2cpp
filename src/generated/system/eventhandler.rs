
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/eventhandler/EventHandler.md")))]
#[::unity2::class(namespace = "System", name = "EventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventHandler {}

#[cfg(feature = "system-eventhandler")]
#[::unity2::methods]
impl EventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();
}

#[cfg(feature = "system-eventhandler")]
impl EventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
