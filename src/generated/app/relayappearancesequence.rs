
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayappearancesequence/RelayAppearanceSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayAppearanceSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: relayappearancesequence :: RelayAppearanceSequence >)]
pub struct RelayAppearanceSequence {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_ReplayAppearanceIndexes")]
    pub m_replay_appearance_indexes: ::unity2::Array<i32>,
    #[rename(name = "m_ReplayLeavingIndexes")]
    pub m_replay_leaving_indexes: ::unity2::Array<i32>,
}

#[cfg(feature = "app-relayappearancesequence")]
#[::unity2::methods]
impl RelayAppearanceSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        appearance_indexes: ::unity2::Array<i32>,
        leaving_indexes: ::unity2::Array<i32>,
    ) -> ();

    #[method(name = "LeavingBegin", args = 0)]
    pub fn leaving_begin(self) -> ();

    #[method(name = "Leaving", args = 0)]
    pub fn leaving(self) -> ();

    #[method(name = "LeavingNext", args = 0)]
    pub fn leaving_next(self) -> ();

    #[method(name = "AppearanceBegin", args = 0)]
    pub fn appearance_begin(self) -> ();

    #[method(name = "Appearance", args = 0)]
    pub fn appearance(self) -> ();

    #[method(name = "AppearanceNext", args = 0)]
    pub fn appearance_next(self) -> ();

    #[method(name = "FocusUnit", args = 0)]
    pub fn focus_unit(self) -> ();

    #[method(name = "GetLeavingUnit", args = 1)]
    pub fn get_leaving_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetAppearanceUnit", args = 1)]
    pub fn get_appearance_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "CreateBindTakeOver", args = 1)]
    pub fn create_bind_take_over(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindReplay", args = 1)]
    pub fn create_bind_replay(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindCommon", args = 2)]
    pub fn create_bind_common(
        p: crate::app::relayappearancesequence::RelayAppearanceSequence,
        super_: crate::app::procinst::ProcInst,
    ) -> ();
}

#[cfg(feature = "app-relayappearancesequence")]
impl RelayAppearanceSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAppearanceSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayAppearanceSequenceMethods>::ctor(this);
        this
    }

    pub fn new_2(
        appearance_indexes: ::unity2::Array<i32>,
        leaving_indexes: ::unity2::Array<i32>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAppearanceSequence),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRelayAppearanceSequenceMethods>::ctor_2(
            this,
            appearance_indexes,
            leaving_indexes,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayappearancesequence/RelayAppearanceSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayAppearanceSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayAppearanceSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAppearanceSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAppearanceSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayAppearanceSequence_Label {
    pub fn leaving() -> Self {
        Self { value: 0 }
    }

    pub fn leaving_loop() -> Self {
        Self { value: 1 }
    }

    pub fn appearance() -> Self {
        Self { value: 2 }
    }

    pub fn appearance_loop() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}
