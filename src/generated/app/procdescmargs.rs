
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesccallbase::IProcDescCallBase;
use crate::app::procdesccallbase::ProcDescCallBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescmargs/ProcDescMArgS.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescMArgS")]
#[parent(crate::app::procdesccallbase::ProcDescCallBase)]
pub struct ProcDescMArgS {
    #[rename(name = "m_Method")]
    pub m_method: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    #[rename(name = "m_Arg")]
    pub m_arg: ::unity2::Il2CppString,
}

#[cfg(feature = "app-procdescmargs")]
#[::unity2::methods]
impl ProcDescMArgS {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        method: crate::system::action_1::Action_1<::unity2::Il2CppString>,
        arg: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdescmargs")]
impl ProcDescMArgS {
    pub fn new(
        method: crate::system::action_1::Action_1<::unity2::Il2CppString>,
        arg: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescMArgS),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescMArgSMethods>::ctor(this, method, arg);
        this
    }
}
