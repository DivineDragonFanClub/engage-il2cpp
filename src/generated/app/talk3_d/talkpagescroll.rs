
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talkpagescroll/TalkPageScroll.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkPageScroll")]
#[parent(crate::app::procinst::ProcInst)]
pub struct TalkPageScroll {}

#[cfg(feature = "app-talk3_d-talkpagescroll")]
#[::unity2::methods]
impl TalkPageScroll {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateInstBind", args = 1)]
    pub fn create_inst_bind(
        parent: crate::app::procinst::ProcInst,
    ) -> crate::app::talk3_d::talkpagescroll::TalkPageScroll;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talkpagescroll")]
impl TalkPageScroll {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkPageScroll),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkPageScrollMethods>::ctor(this);
        this
    }
}
