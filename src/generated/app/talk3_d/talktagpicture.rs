
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagpicture/TalkTagPicture.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagPicture")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagPicture {
    #[rename(name = "m_TagID")]
    pub m_tag_id: crate::app::mess::Mess_TagID_Picture,
    #[rename(name = "m_PictureIndex")]
    pub m_picture_index: i32,
    #[rename(name = "m_TextureName")]
    pub m_texture_name: ::unity2::Il2CppString,
    #[rename(name = "m_AnimName")]
    pub m_anim_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-talk3_d-talktagpicture")]
#[::unity2::methods]
impl TalkTagPicture {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, talk_ptr: crate::app::talk3_d::talkptr::TalkPtr) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagpicture")]
impl TalkTagPicture {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagPicture),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagPictureMethods>::ctor(this);
        this
    }
}
