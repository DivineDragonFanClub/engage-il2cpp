
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencereplay/MapSequenceReplay_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceReplay_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceReplay_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceReplay.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceReplay_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceReplay_Label {
    pub fn read() -> Self {
        Self { value: 0 }
    }

    pub fn mind() -> Self {
        Self { value: 1 }
    }

    pub fn engage_start() -> Self {
        Self { value: 2 }
    }

    pub fn engage_link() -> Self {
        Self { value: 3 }
    }

    pub fn engage_rewarp() -> Self {
        Self { value: 4 }
    }

    pub fn god_change() -> Self {
        Self { value: 5 }
    }

    pub fn surrender() -> Self {
        Self { value: 6 }
    }

    pub fn cancel() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencereplay/MapSequenceReplay.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceReplay")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapsequencereplay :: MapSequenceReplay >)]
pub struct MapSequenceReplay {}

#[cfg(feature = "app-mapsequencereplay")]
#[::unity2::methods]
impl MapSequenceReplay {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "Read", args = 0)]
    pub fn read(self) -> ();

    #[method(name = "Focus", args = 0)]
    pub fn focus(self) -> ();

    #[method(name = "MindTypeBranch", args = 0)]
    pub fn mind_type_branch(self) -> ();

    #[method(name = "PrepareEngageRewarp", args = 0)]
    pub fn prepare_engage_rewarp(self) -> ();

    #[method(name = "Mind", args = 0)]
    pub fn mind(self) -> ();

    #[method(name = "Surrender", args = 0)]
    pub fn surrender(self) -> ();

    #[method(name = "GameEndBranch", args = 0)]
    pub fn game_end_branch(self) -> ();

    #[method(name = "CheckCancelInput", args = 0)]
    pub fn check_cancel_input(self) -> ();

    #[method(name = "CheckCancel", args = 0)]
    pub fn check_cancel(self) -> bool;

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "FinishReplay", args = 0)]
    pub fn finish_replay(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencereplay")]
impl MapSequenceReplay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceReplay),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceReplayMethods>::ctor(this);
        this
    }
}
