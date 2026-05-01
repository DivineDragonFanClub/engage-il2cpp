
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceengagesummon/MapSequenceEngageSummon_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceEngageSummon_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceEngageSummon_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceEngageSummon.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceEngageSummon_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceEngageSummon_Label {
    pub fn simple() -> Self {
        Self { value: 0 }
    }

    pub fn detail() -> Self {
        Self { value: 1 }
    }

    pub fn after() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequenceengagesummon/MapSequenceEngageSummon.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceEngageSummon")]
# [parent (crate :: app :: commonbattlesequence_1 :: CommonBattleSequence_1 < crate :: app :: mapsequenceengagesummon :: MapSequenceEngageSummon >)]
pub struct MapSequenceEngageSummon {}

#[cfg(feature = "app-mapsequenceengagesummon")]
#[::unity2::methods]
impl MapSequenceEngageSummon {
    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: crate::app::persondata::PersonData) -> ();

    #[method(name = "get_Rank", args = 0)]
    pub fn get_rank(self) -> crate::app::persondata::PersonData_Ranks;

    #[method(name = "set_Rank", args = 1)]
    pub fn set_rank(self, value: crate::app::persondata::PersonData_Ranks) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "MindStart", args = 0)]
    pub fn mind_start(self) -> ();

    #[method(name = "MindEnd", args = 0)]
    pub fn mind_end(self) -> ();

    #[method(name = "BgmStart", args = 0)]
    pub fn bgm_start(self) -> ();

    #[method(name = "BgmEnd", args = 0)]
    pub fn bgm_end(self) -> ();

    #[method(name = "Calculate", args = 0)]
    pub fn calculate(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "SimpleSummon", args = 0)]
    pub fn simple_summon(self) -> ();

    #[method(name = "CombatSummon", args = 0)]
    pub fn combat_summon(self) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Grow", args = 0)]
    pub fn grow(self) -> ();

    #[method(name = "GodExp", args = 0)]
    pub fn god_exp(self) -> ();

    #[method(name = "TrySkip", args = 0)]
    pub fn try_skip(self) -> ();

    #[method(name = "CreateTelop", args = 0)]
    pub fn create_telop(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mapsequenceengagesummon")]
impl MapSequenceEngageSummon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceEngageSummon),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceEngageSummonMethods>::ctor(this);
        this
    }
}
