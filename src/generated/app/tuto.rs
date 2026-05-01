
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdescuser::IProcDescUser;
use crate::app::procdescuser::ProcDescUser;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tuto/Tuto_ProcDescTutorial.md")))]
#[::unity2::class(namespace = "App", name = "Tuto.ProcDescTutorial")]
#[parent(crate::app::procdescuser::ProcDescUser)]
pub struct Tuto_ProcDescTutorial {
    #[rename(name = "m_Tid")]
    pub m_tid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-tuto")]
#[::unity2::methods]
impl Tuto_ProcDescTutorial {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-tuto")]
impl Tuto_ProcDescTutorial {
    pub fn new(tid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Tuto_ProcDescTutorial),
                ::core::stringify!(new),
            )
        });
        <Self as ITuto_ProcDescTutorialMethods>::ctor(this, tid);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tuto/Tuto.md")))]
#[::unity2::class(namespace = "App", name = "Tuto")]
#[parent(crate::system::object::Object)]
pub struct Tuto {}

#[cfg(feature = "app-tuto")]
#[::unity2::methods]
impl Tuto {
    #[method(name = "Call", args = 1)]
    pub fn call(tid: ::unity2::Il2CppString) -> crate::app::procdesc::ProcDesc;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-tuto")]
impl Tuto {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Tuto),
                ::core::stringify!(new),
            )
        });
        <Self as ITutoMethods>::ctor(this);
        this
    }
}
