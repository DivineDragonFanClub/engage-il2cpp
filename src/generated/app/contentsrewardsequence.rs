
use crate::app::irewardsequence::IIRewardSequence;
use crate::app::irewardsequence::IRewardSequence;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/contentsrewardsequence/ContentsRewardSequence.md")))]
#[::unity2::class(namespace = "App", name = "ContentsRewardSequence")]
#[parent(crate::app::irewardsequence::IRewardSequence)]
pub struct ContentsRewardSequence {}

#[cfg(feature = "app-contentsrewardsequence")]
#[::unity2::methods]
impl ContentsRewardSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "RewordAll", args = 0)]
    pub fn reword_all(self) -> ();

    #[method(name = "GetDesc", args = 0)]
    pub fn get_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-contentsrewardsequence")]
impl ContentsRewardSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ContentsRewardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IContentsRewardSequenceMethods>::ctor(this);
        this
    }
}
