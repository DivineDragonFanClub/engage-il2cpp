
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procdescjump/ProcDescJump.md")))]
#[::unity2::class(namespace = "App", name = "ProcDescJump")]
#[parent(crate::app::procdesc::ProcDesc)]
pub struct ProcDescJump {
    #[rename(name = "m_Label")]
    pub m_label: i32,
}

#[cfg(feature = "app-procdescjump")]
#[::unity2::methods]
impl ProcDescJump {
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

#[cfg(feature = "app-procdescjump")]
impl ProcDescJump {
    pub fn new(label: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcDescJump),
                ::core::stringify!(new),
            )
        });
        <Self as IProcDescJumpMethods>::ctor(this, label);
        this
    }
}
