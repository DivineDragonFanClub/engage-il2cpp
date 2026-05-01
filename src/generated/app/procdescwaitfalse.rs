
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdescwaitfunc::IProcDescWaitFunc;
use crate::app::procdescwaitfunc::ProcDescWaitFunc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescwaitfalse/ProcDescWaitFalse.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescWaitFalse")]
#[parent(crate::app::procdescwaitfunc::ProcDescWaitFunc)]
pub struct ProcDescWaitFalse {
    #[rename(name = "m_Function")]
    pub m_function: crate::app::procboolfunction::ProcBoolFunction,
}

#[cfg(feature = "app-procdescwaitfalse")]
#[::unity2::methods]
impl ProcDescWaitFalse {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, function: crate::app::procboolfunction::ProcBoolFunction) -> ();

    #[method(name = "IsWait", args = 1)]
    pub fn is_wait(self, inst: crate::app::procinst::ProcInst) -> bool;
}

#[cfg(feature = "app-procdescwaitfalse")]
impl ProcDescWaitFalse {
    pub fn new(function: crate::app::procboolfunction::ProcBoolFunction) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescWaitFalse),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescWaitFalseMethods>::ctor(this, function);
        this
    }
}
