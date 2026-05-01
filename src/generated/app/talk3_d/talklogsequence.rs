
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogsequence/TalkLogSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TalkLogSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TalkLogSequence_Label {
    const NAMESPACE: &'static str = "App.Talk3D";

    const NAME: &'static str = "TalkLogSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TalkLogSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TalkLogSequence_Label {
    pub fn fade_in() -> Self {
        Self { value: 0 }
    }

    pub fn tick() -> Self {
        Self { value: 1 }
    }

    pub fn fade_out() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talk3_d/talklogsequence/TalkLogSequence.md")))]
#[::unity2::class(namespace = "App.Talk3D", name = "TalkLogSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: talk3_d :: talklogsequence :: TalkLogSequence >)]
pub struct TalkLogSequence {
    #[rename(name = "m_IsPlayedVoice")]
    pub m_is_played_voice: bool,
}

#[cfg(feature = "app-talk3_d-talklogsequence")]
#[::unity2::methods]
impl TalkLogSequence {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "ShowTalkUI", args = 0)]
    pub fn show_talk_ui(self) -> ();

    #[method(name = "HideTalkUI", args = 0)]
    pub fn hide_talk_ui(self) -> ();

    #[method(name = "StartFadeIn", args = 0)]
    pub fn start_fade_in(self) -> ();

    #[method(name = "WaitFadeIn", args = 0)]
    pub fn wait_fade_in(self) -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "StopVoice", args = 0)]
    pub fn stop_voice(self) -> ();

    #[method(name = "StartFadeOut", args = 0)]
    pub fn start_fade_out(self) -> ();

    #[method(name = "WaitFadeOut", args = 0)]
    pub fn wait_fade_out(self) -> bool;

    #[method(name = "EndFadeOut", args = 0)]
    pub fn end_fade_out(self) -> ();

    #[method(name = "GetDesc", args = 0)]
    pub fn get_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        parent: crate::app::procinst::ProcInst,
    ) -> crate::app::talk3_d::talklogsequence::TalkLogSequence;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talk3_d-talklogsequence")]
impl TalkLogSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkLogSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkLogSequenceMethods>::ctor(this);
        this
    }
}
