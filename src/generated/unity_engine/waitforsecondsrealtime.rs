
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::customyieldinstruction::CustomYieldInstruction;
use crate::unity_engine::customyieldinstruction::ICustomYieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/waitforsecondsrealtime/WaitForSecondsRealtime.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "WaitForSecondsRealtime")]
#[parent(crate::unity_engine::customyieldinstruction::CustomYieldInstruction)]
pub struct WaitForSecondsRealtime {
    #[rename(name = "m_WaitUntilTime")]
    pub m_wait_until_time: f32,
}

#[cfg(feature = "unity_engine-waitforsecondsrealtime")]
#[::unity2::methods]
impl WaitForSecondsRealtime {
    #[method(name = "get_waitTime", args = 0)]
    pub fn get_wait_time(self) -> f32;

    #[method(name = "set_waitTime", args = 1)]
    pub fn set_wait_time(self, value: f32) -> ();

    #[method(name = "get_keepWaiting", args = 0)]
    pub fn get_keep_waiting(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, time: f32) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "unity_engine-waitforsecondsrealtime")]
impl WaitForSecondsRealtime {
    pub fn new(time: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaitForSecondsRealtime),
                ::core::stringify!(new),
            )
        });
        <Self as IWaitForSecondsRealtimeMethods>::ctor(this, time);
        this
    }
}
