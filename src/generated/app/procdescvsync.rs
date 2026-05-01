
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesccallbase::IProcDescCallBase;
use crate::app::procdesccallbase::ProcDescCallBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescvsync/ProcDescVsync.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescVsync")]
#[parent(crate::app::procdesccallbase::ProcDescCallBase)]
pub struct ProcDescVsync {
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::gametime::GameTime_VsycMode,
}

#[cfg(feature = "app-procdescvsync")]
#[::unity2::methods]
impl ProcDescVsync {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, mode: crate::app::gametime::GameTime_VsycMode) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdescvsync")]
impl ProcDescVsync {
    pub fn new(mode: crate::app::gametime::GameTime_VsycMode) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescVsync),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescVsyncMethods>::ctor(this, mode);
        this
    }
}
