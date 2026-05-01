
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatworlddisposer/CombatWorldDisposer.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatWorldDisposer")]
#[parent(crate::system::object::Object)]
pub struct CombatWorldDisposer {}

#[cfg(feature = "combat-combatworlddisposer")]
#[::unity2::methods]
impl CombatWorldDisposer {
    #[method(name = "Startup", args = 0)]
    pub fn startup() -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose() -> ();

    #[method(name = "Shutdown", args = 0)]
    pub fn shutdown() -> ();
}
