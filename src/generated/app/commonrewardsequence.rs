
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewardsequence/CommonRewardSequence_ProcDiscardMessage.md")))]
#[::unity2::class(namespace = "App", name = "CommonRewardSequence.ProcDiscardMessage")]
#[parent(crate::app::procinst::ProcInst)]
pub struct CommonRewardSequence_ProcDiscardMessage {}

#[cfg(feature = "app-commonrewardsequence")]
#[::unity2::methods]
impl CommonRewardSequence_ProcDiscardMessage {
    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> crate::app::procinst::ProcInst;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-commonrewardsequence")]
impl CommonRewardSequence_ProcDiscardMessage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRewardSequence_ProcDiscardMessage),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRewardSequence_ProcDiscardMessageMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewardsequence/CommonRewardSequence.md")))]
#[::unity2::class(namespace = "App", name = "CommonRewardSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct CommonRewardSequence {
    #[rename(name = "m_Bg")]
    pub m_bg: crate::app::menubg::MenuBg,
    #[rename(name = "m_RewardExpList")]
    pub m_reward_exp_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::unit::Unit,
        i32,
    >,
    #[rename(name = "m_RewardItemList")]
    pub m_reward_item_list:
        crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    #[rename(name = "m_RewardMoney")]
    pub m_reward_money: i32,
    #[rename(name = "m_IsDiscard")]
    pub m_is_discard: bool,
    #[rename(name = "m_IsCreateBg")]
    pub m_is_create_bg: bool,
    #[rename(name = "m_IsClear")]
    pub m_is_clear: bool,
    #[rename(name = "TitleMID")]
    pub title_mid: ::unity2::Il2CppString,
    #[rename(name = "m_LevelUpUnitList")]
    pub m_level_up_unit_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::unit::Unit,
        i32,
    >,
}

#[cfg(feature = "app-commonrewardsequence")]
#[::unity2::methods]
impl CommonRewardSequence {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        reward_exp_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::unit::Unit,
            i32,
        >,
        reward_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        reward_money: i32,
        is_create_bg: bool,
    ) -> ();

    #[method(name = "CreateBindClear", args = 2)]
    pub fn create_bind_clear(
        super_: crate::app::procinst::ProcInst,
        reward_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
    ) -> ();

    #[method(name = "CreateBindForWell", args = 3)]
    pub fn create_bind_for_well(
        super_: crate::app::procinst::ProcInst,
        reward_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        title: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        reward_exp_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::unit::Unit,
            i32,
        >,
        reward_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        reward_money: i32,
        is_create_bg: bool,
        is_clear: bool,
    ) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateDescForWell", args = 0)]
    pub fn create_desc_for_well(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "OpenRewardExp", args = 0)]
    pub fn open_reward_exp(self) -> ();

    #[method(name = "CheckLevelUp", args = 0)]
    pub fn check_level_up(self) -> ();

    #[method(name = "TryCreateBindDiscardMessage", args = 2)]
    pub fn try_create_bind_discard_message(
        super_: crate::app::procinst::ProcInst,
        discard: bool,
    ) -> crate::app::procinst::ProcInst;

    #[method(name = "CalcRewardItemList", args = 3)]
    pub fn calc_reward_item_list(
        item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        money: i32,
        discard: bool,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::itemdata::ItemData,
        i32,
    >;

    #[method(name = "CalcRewardItemList", args = 0)]
    pub fn calc_reward_item_list_2(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::itemdata::ItemData,
        i32,
    >;

    #[method(name = "OpenRewardItem", args = 0)]
    pub fn open_reward_item(self) -> ();

    #[method(name = "OpenRewardItemForWell", args = 0)]
    pub fn open_reward_item_for_well(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "CheckItemOverflow", args = 0)]
    pub fn check_item_overflow(self) -> ();
}

#[cfg(feature = "app-commonrewardsequence")]
impl CommonRewardSequence {
    pub fn new(
        reward_exp_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::unit::Unit,
            i32,
        >,
        reward_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemdata::ItemData,
        >,
        reward_money: i32,
        is_create_bg: bool,
        is_clear: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRewardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRewardSequenceMethods>::ctor(
            this,
            reward_exp_list,
            reward_item_list,
            reward_money,
            is_create_bg,
            is_clear,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewardsequence/CommonRewardSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CommonRewardSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CommonRewardSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "CommonRewardSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CommonRewardSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CommonRewardSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn check_level_up() -> Self {
        Self { value: 1 }
    }

    pub fn close() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}
