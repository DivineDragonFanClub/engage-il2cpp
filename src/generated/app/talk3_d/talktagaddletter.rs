
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagaddletter/TalkTagAddLetter.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagAddLetter")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagAddLetter {
    #[rename(name = "m_IsLineFeedEnable")]
    pub m_is_line_feed_enable: bool,
    #[rename(name = "m_AddLetter")]
    pub m_add_letter: u16,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::talk3_d::talktag::TalkTag_Result,
}

#[cfg(feature = "app-talk3_d-talktagaddletter")]
#[::unity2::methods]
impl TalkTagAddLetter {
    #[method(name = "SetLetter", args = 1)]
    pub fn set_letter(self, chr: u16) -> ();

    #[method(name = "ResetLineFeedEnable", args = 0)]
    pub fn reset_line_feed_enable(self) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "ExecuteForCharacterPreLoad", args = 0)]
    pub fn execute_for_character_pre_load(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = "GetResultForHeadText", args = 0)]
    pub fn get_result_for_head_text(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = "GetAddLetter", args = 0)]
    pub fn get_add_letter(self) -> u16;

    #[method(name = "NeedFadeWait", args = 0)]
    pub fn need_fade_wait(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagaddletter")]
impl TalkTagAddLetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagAddLetter),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagAddLetterMethods>::ctor(this);
        this
    }
}
