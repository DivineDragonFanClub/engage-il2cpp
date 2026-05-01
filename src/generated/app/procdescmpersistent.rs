
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescmpersistent/ProcDescMPersistent.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescMPersistent")]
#[parent(crate::app::procdesc::ProcDesc)]
pub struct ProcDescMPersistent {
    #[rename(name = "m_Method")]
    pub m_method: crate::app::procvoidmethod::ProcVoidMethod,
}

#[cfg(feature = "app-procdescmpersistent")]
#[::unity2::methods]
impl ProcDescMPersistent {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, method: crate::app::procvoidmethod::ProcVoidMethod) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-procdescmpersistent")]
impl ProcDescMPersistent {
    pub fn new(method: crate::app::procvoidmethod::ProcVoidMethod) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescMPersistent),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescMPersistentMethods>::ctor(this, method);
        this
    }
}
