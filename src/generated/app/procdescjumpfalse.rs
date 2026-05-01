
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdescjumpfunc::IProcDescJumpFunc;
use crate::app::procdescjumpfunc::ProcDescJumpFunc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescjumpfalse/ProcDescJumpFalse.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescJumpFalse")]
#[parent(crate::app::procdescjumpfunc::ProcDescJumpFunc)]
pub struct ProcDescJumpFalse {
    #[rename(name = "m_Function")]
    pub m_function: crate::app::procboolfunction::ProcBoolFunction,
}

#[cfg(feature = "app-procdescjumpfalse")]
#[::unity2::methods]
impl ProcDescJumpFalse {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, function: crate::app::procboolfunction::ProcBoolFunction, label: i32) -> ();

    #[method(name = "IsJump", args = 1)]
    pub fn is_jump(self, inst: crate::app::procinst::ProcInst) -> bool;
}

#[cfg(feature = "app-procdescjumpfalse")]
impl ProcDescJumpFalse {
    pub fn new(function: crate::app::procboolfunction::ProcBoolFunction, label: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescJumpFalse),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescJumpFalseMethods>::ctor(this, function, label);
        this
    }
}
