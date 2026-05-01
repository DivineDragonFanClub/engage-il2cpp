
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endingsequence/EndingSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EndingSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EndingSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EndingSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EndingSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EndingSequence_Label {
    pub fn battle_record() -> Self {
        Self { value: 0 }
    }

    pub fn later_talk() -> Self {
        Self { value: 1 }
    }

    pub fn end_roll() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/endingsequence/EndingSequence.md")))]
#[::unity2::class(namespace = "App", name = "EndingSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct EndingSequence {}

#[cfg(feature = "app-endingsequence")]
#[::unity2::methods]
impl EndingSequence {
    #[method(name = "get_BGMHeader1", args = 0)]
    pub fn get_bgm_header1(self) -> ::unity2::Il2CppString;

    #[method(name = "get_BGMHeader2", args = 0)]
    pub fn get_bgm_header2(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "PlayMovie", args = 0)]
    pub fn play_movie(self) -> ();

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "TearDown", args = 0)]
    pub fn tear_down(self) -> ();

    #[method(name = "StartBattleRecord", args = 0)]
    pub fn start_battle_record(self) -> ();

    #[method(name = "StartLaterTalk", args = 0)]
    pub fn start_later_talk(self) -> ();

    #[method(name = "StartEndRoll", args = 0)]
    pub fn start_end_roll(self) -> ();

    #[method(name = "SaveMenu", args = 0)]
    pub fn save_menu(self) -> ();

    #[method(name = "EnableControllerSupport", args = 0)]
    pub fn enable_controller_support(self) -> ();

    #[method(name = "DisableControllerSupport", args = 0)]
    pub fn disable_controller_support(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-endingsequence")]
impl EndingSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EndingSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IEndingSequenceMethods>::ctor(this);
        this
    }
}
