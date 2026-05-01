
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/proceventwait/ProcEventWait.md")))]
#[::unity2::class(namespace = "App", name = "ProcEventWait")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProcEventWait {
    #[rename(name = "m_Tick")]
    pub m_tick: f32,
    #[rename(name = "m_Time")]
    pub m_time: f32,
}

#[cfg(feature = "app-proceventwait")]
#[::unity2::methods]
impl ProcEventWait {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, time: f32) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, time: f32) -> ();
}

#[cfg(feature = "app-proceventwait")]
impl ProcEventWait {
    pub fn new(time: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcEventWait),
                ::core::stringify!(new),
            )
        });
        <Self as IProcEventWaitMethods>::ctor(this, time);
        this
    }
}
