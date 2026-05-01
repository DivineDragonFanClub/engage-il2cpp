
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawardsequence/RelayAwardSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayAwardSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayAwardSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAwardSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAwardSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayAwardSequence_Label {
    pub fn unit_award() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawardsequence/RelayAwardSequence.md")))]
#[::unity2::class(namespace = "App", name = "RelayAwardSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: relayawardsequence :: RelayAwardSequence >)]
pub struct RelayAwardSequence {
    #[rename(name = "m_EnteredBattle")]
    pub m_entered_battle: crate::app::relayuserdata::RelayUserData_EnteredBattle,
    #[rename(name = "m_Raids")]
    pub m_raids: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_GainItem")]
    pub m_gain_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_IsAwarded")]
    pub m_is_awarded: bool,
    #[rename(name = "m_PrevMasterProofCount")]
    pub m_prev_master_proof_count: i32,
    #[rename(name = "m_PrevChangeProofCount")]
    pub m_prev_change_proof_count: i32,
}

#[cfg(feature = "app-relayawardsequence")]
#[::unity2::methods]
impl RelayAwardSequence {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "TeamAward", args = 0)]
    pub fn team_award(self) -> ();

    #[method(name = "UnitAward", args = 0)]
    pub fn unit_award(self) -> ();

    #[method(name = "GainItem", args = 0)]
    pub fn gain_item(self) -> ();

    #[method(name = "GainItems", args = 1)]
    pub fn gain_items(
        self,
        result: crate::app::relaycompletionawarddata::RelayCompletionAwardData_CalcResult,
    ) -> ();

    #[method(name = "GainItems", args = 1)]
    pub fn gain_items_2(
        self,
        items: crate::system::collections::generic::list_1::List_1<
            crate::app::relaycompletionawarddata::RelayCompletionAwardData_CalcResult_Item,
        >,
    ) -> ();

    #[method(name = "SetGainItem", args = 1)]
    pub fn set_gain_item(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "TutorialClassChange", args = 0)]
    pub fn tutorial_class_change(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetEnteredBattle", args = 0)]
    pub fn get_entered_battle() -> crate::app::relayuserdata::RelayUserData_EnteredBattle;

    #[method(name = "GetEnteredBattleFromUserData", args = 0)]
    pub fn get_entered_battle_from_user_data(
    ) -> crate::app::relayuserdata::RelayUserData_EnteredBattle;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayawardsequence")]
impl RelayAwardSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAwardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayAwardSequenceMethods>::ctor(this);
        this
    }
}
