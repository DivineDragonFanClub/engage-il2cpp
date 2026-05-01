
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdesctickbase::IProcDescTickBase;
use crate::app::procdesctickbase::ProcDescTickBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescmtickenum/ProcDescMTickEnum.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescMTickEnum")]
#[parent(crate::app::procdesctickbase::ProcDescTickBase)]
pub struct ProcDescMTickEnum {
    #[rename(name = "m_SrcMethod")]
    pub m_src_method: crate::app::procenummethod::ProcEnumMethod,
    #[rename(name = "m_Method")]
    pub m_method: crate::system::collections::ienumerator::IEnumerator,
    #[rename(name = "m_IsFinished")]
    pub m_is_finished: bool,
}

#[cfg(feature = "app-procdescmtickenum")]
#[::unity2::methods]
impl ProcDescMTickEnum {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, method: crate::app::procenummethod::ProcEnumMethod) -> ();

    #[method(name = "ExecuteImpl", args = 1)]
    pub fn execute_impl(self, inst: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-procdescmtickenum")]
impl ProcDescMTickEnum {
    pub fn new(method: crate::app::procenummethod::ProcEnumMethod) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescMTickEnum),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescMTickEnumMethods>::ctor(this, method);
        this
    }
}
