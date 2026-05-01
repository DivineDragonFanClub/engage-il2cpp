
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescpush/ProcDescPush.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescPush")]
#[parent(crate::app::procdesc::ProcDesc)]
pub struct ProcDescPush {
    #[rename(name = "m_Label")]
    pub m_label: i32,
}

#[cfg(feature = "app-procdescpush")]
#[::unity2::methods]
impl ProcDescPush {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, label: i32) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> i32;
}

#[cfg(feature = "app-procdescpush")]
impl ProcDescPush {
    pub fn new(label: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescPush),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescPushMethods>::ctor(this, label);
        this
    }
}
