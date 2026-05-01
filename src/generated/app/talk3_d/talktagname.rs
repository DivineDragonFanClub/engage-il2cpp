
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagname/TalkTagName.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagName")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagName {
    #[rename(name = "m_TagID")]
    pub m_tag_id: crate::app::mess::Mess_TagID_Name,
    #[rename(name = "m_Pid")]
    pub m_pid: ::unity2::Il2CppString,
    #[rename(name = "m_ReplaceName")]
    pub m_replace_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-talk3_d-talktagname")]
#[::unity2::methods]
impl TalkTagName {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, talk_ptr: crate::app::talk3_d::talkptr::TalkPtr) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = "GetTagID", args = 0)]
    pub fn get_tag_id(self) -> crate::app::mess::Mess_TagID_Name;

    #[method(name = "GetReplaceName", args = 0)]
    pub fn get_replace_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagname")]
impl TalkTagName {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagName),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagNameMethods>::ctor(this);
        this
    }
}
