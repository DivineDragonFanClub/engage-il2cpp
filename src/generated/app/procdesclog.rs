
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesccallbase::IProcDescCallBase;
use crate::app::procdesccallbase::ProcDescCallBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdesclog/ProcDescLog.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescLog")]
#[parent(crate::app::procdesccallbase::ProcDescCallBase)]
pub struct ProcDescLog {
    #[rename(name = "m_Log")]
    pub m_log: ::unity2::Il2CppString,
}

#[cfg(feature = "app-procdesclog")]
#[::unity2::methods]
impl ProcDescLog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, log: ::unity2::Il2CppString) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdesclog")]
impl ProcDescLog {
    pub fn new(log: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescLog),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescLogMethods>::ctor(this, log);
        this
    }
}
