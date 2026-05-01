
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/customyieldinstruction/CustomYieldInstruction.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CustomYieldInstruction")]
#[parent(crate::system::object::Object)]
pub struct CustomYieldInstruction {}

#[cfg(feature = "unity_engine-customyieldinstruction")]
#[::unity2::methods]
impl CustomYieldInstruction {
    #[method(name = "get_keepWaiting", args = 0)]
    pub fn get_keep_waiting(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-customyieldinstruction")]
impl CustomYieldInstruction {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomYieldInstruction),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomYieldInstructionMethods>::ctor(this);
        this
    }
}
