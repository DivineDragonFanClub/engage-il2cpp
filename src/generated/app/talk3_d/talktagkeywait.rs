
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagkeywait/TalkTagKeyWait.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagKeyWait")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagKeyWait {
    #[rename(name = "m_TagID")]
    pub m_tag_id: crate::app::talk3_d::talktagkeywait::TalkTagKeyWait_TagID,
    #[rename(name = "m_Sec")]
    pub m_sec: f32,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::talk3_d::talktag::TalkTag_Result,
}

#[cfg(feature = "app-talk3_d-talktagkeywait")]
#[::unity2::methods]
impl TalkTagKeyWait {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, talk_ptr: crate::app::talk3_d::talkptr::TalkPtr) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagkeywait")]
impl TalkTagKeyWait {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagKeyWait),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagKeyWaitMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagkeywait/TalkTagKeyWait_TagID.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TalkTagKeyWait_TagID {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TalkTagKeyWait_TagID {
    const NAMESPACE: &'static str = "App.Talk3D";

    const NAME: &'static str = "TalkTagKeyWait.TagID";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TalkTagKeyWait_TagID {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TalkTagKeyWait_TagID {
    pub fn key() -> Self {
        Self { value: 0 }
    }

    pub fn page() -> Self {
        Self { value: 1 }
    }

    pub fn key_page() -> Self {
        Self { value: 2 }
    }

    pub fn time_wait() -> Self {
        Self { value: 3 }
    }
}
