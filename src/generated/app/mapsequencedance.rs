
use crate::app::commonbattlesequence_1::CommonBattleSequence_1;
use crate::app::commonbattlesequence_1::ICommonBattleSequence_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencedance/MapSequenceDance.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceDance")]
# [parent (crate :: app :: commonbattlesequence_1 :: CommonBattleSequence_1 < crate :: app :: mapsequencedance :: MapSequenceDance >)]
pub struct MapSequenceDance {}

#[cfg(feature = "app-mapsequencedance")]
#[::unity2::methods]
impl MapSequenceDance {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Calculate", args = 0)]
    pub fn calculate(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "SimpleDance", args = 0)]
    pub fn simple_dance(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "CombatDance", args = 0)]
    pub fn combat_dance(self) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Grow", args = 0)]
    pub fn grow(self) -> ();

    #[method(name = "GodExp", args = 0)]
    pub fn god_exp(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mapsequencedance")]
impl MapSequenceDance {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceDance),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceDanceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencedance/MapSequenceDance_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceDance_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceDance_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceDance.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceDance_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceDance_Label {
    pub fn simple() -> Self {
        Self { value: 0 }
    }

    pub fn detail() -> Self {
        Self { value: 1 }
    }

    pub fn skip() -> Self {
        Self { value: 2 }
    }

    pub fn after() -> Self {
        Self { value: 3 }
    }
}
