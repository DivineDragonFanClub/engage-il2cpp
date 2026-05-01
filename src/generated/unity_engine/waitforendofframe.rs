
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/waitforendofframe/WaitForEndOfFrame.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "WaitForEndOfFrame")]
#[parent(crate::unity_engine::yieldinstruction::YieldInstruction)]
pub struct WaitForEndOfFrame {}

#[cfg(feature = "unity_engine-waitforendofframe")]
#[::unity2::methods]
impl WaitForEndOfFrame {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-waitforendofframe")]
impl WaitForEndOfFrame {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaitForEndOfFrame),
                ::core::stringify!(new),
            )
        });
        <Self as IWaitForEndOfFrameMethods>::ctor(this);
        this
    }
}
