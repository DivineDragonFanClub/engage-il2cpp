
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagarg/TalkTagArg.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagArg")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagArg {
    #[rename(name = "m_ArgIndex")]
    pub m_arg_index: i32,
}

#[cfg(feature = "app-talk3_d-talktagarg")]
#[::unity2::methods]
impl TalkTagArg {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, talk_ptr: crate::app::talk3_d::talkptr::TalkPtr) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagarg")]
impl TalkTagArg {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagArg),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagArgMethods>::ctor(this);
        this
    }
}
