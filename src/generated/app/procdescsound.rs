
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesccallbase::IProcDescCallBase;
use crate::app::procdesccallbase::ProcDescCallBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescsound/ProcDescSound.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescSound")]
#[parent(crate::app::procdesccallbase::ProcDescCallBase)]
pub struct ProcDescSound {
    #[rename(name = "m_EventName")]
    pub m_event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-procdescsound")]
#[::unity2::methods]
impl ProcDescSound {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdescsound")]
impl ProcDescSound {
    pub fn new(event_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescSound),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescSoundMethods>::ctor(this, event_name);
        this
    }
}
