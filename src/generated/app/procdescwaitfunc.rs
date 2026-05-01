
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescwaitfunc/ProcDescWaitFunc.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescWaitFunc")]
#[parent(crate::app::procdesc::ProcDesc)]
pub struct ProcDescWaitFunc {}

#[cfg(feature = "app-procdescwaitfunc")]
#[::unity2::methods]
impl ProcDescWaitFunc {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsWait", args = 1)]
    pub fn is_wait(self, inst: crate::app::procinst::ProcInst) -> bool;

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-procdescwaitfunc")]
impl ProcDescWaitFunc {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescWaitFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescWaitFuncMethods>::ctor(this);
        this
    }
}
