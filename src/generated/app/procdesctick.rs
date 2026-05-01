
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesctickbase::IProcDescTickBase;
use crate::app::procdesctickbase::ProcDescTickBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdesctick/ProcDescTick.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescTick")]
#[parent(crate::app::procdesctickbase::ProcDescTickBase)]
pub struct ProcDescTick {
    #[rename(name = "m_Function")]
    pub m_function: crate::app::procvoidfunction::ProcVoidFunction,
}

#[cfg(feature = "app-procdesctick")]
#[::unity2::methods]
impl ProcDescTick {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, function: crate::app::procvoidfunction::ProcVoidFunction) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdesctick")]
impl ProcDescTick {
    pub fn new(function: crate::app::procvoidfunction::ProcVoidFunction) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescTick),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescTickMethods>::ctor(this, function);
        this
    }
}
