
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/irewardsequence/IRewardSequence.md")))]
#[::unity2::class(namespace = "App", name = "IRewardSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct IRewardSequence {}

#[cfg(feature = "app-irewardsequence")]
#[::unity2::methods]
impl IRewardSequence {
    #[method(name = "GetDesc", args = 0)]
    pub fn get_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-irewardsequence")]
impl IRewardSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IRewardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IIRewardSequenceMethods>::ctor(this);
        this
    }
}
