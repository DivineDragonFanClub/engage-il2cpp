
use crate::app::talk3_d::talktag::ITalkTag;
use crate::app::talk3_d::talktag::TalkTag;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagicon/TalkTagIcon_TagID.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TalkTagIcon_TagID {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TalkTagIcon_TagID {
    const NAMESPACE: &'static str = "App.Talk3D";

    const NAME: &'static str = "TalkTagIcon.TagID";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TalkTagIcon_TagID {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TalkTagIcon_TagID {
    pub fn item() -> Self {
        Self { value: 0 }
    }

    pub fn skill() -> Self {
        Self { value: 1 }
    }

    pub fn system() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talktagicon/TalkTagIcon.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkTagIcon")]
#[parent(crate::app::talk3_d::talktag::TalkTag)]
pub struct TalkTagIcon {
    #[rename(name = "m_TagID")]
    pub m_tag_id: crate::app::talk3_d::talktagicon::TalkTagIcon_TagID,
    #[rename(name = "m_KindName")]
    pub m_kind_name: ::unity2::Il2CppString,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::talk3_d::talktag::TalkTag_Result,
}

#[cfg(feature = "app-talk3_d-talktagicon")]
#[::unity2::methods]
impl TalkTagIcon {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, talk_ptr: crate::app::talk3_d::talkptr::TalkPtr) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::talk3_d::talktag::TalkTag_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talktagicon")]
impl TalkTagIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkTagIcon),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkTagIconMethods>::ctor(this);
        this
    }
}
