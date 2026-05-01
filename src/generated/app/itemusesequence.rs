
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemusesequence/ItemUseSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ItemUseSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ItemUseSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ItemUseSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ItemUseSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ItemUseSequence_Label {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn class_change() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemusesequence/ItemUseSequence.md")))]
#[::unity2::class(namespace = "App", name = "ItemUseSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ItemUseSequence {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UseItem")]
    pub m_use_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_ItemData")]
    pub m_item_data: crate::app::itemdata::ItemData,
    #[rename(name = "m_TargetIndex")]
    pub m_target_index: i32,
    #[rename(name = "m_GainExp")]
    pub m_gain_exp: i32,
}

#[cfg(feature = "app-itemusesequence")]
#[::unity2::methods]
impl ItemUseSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "GetTargetUnit", args = 0)]
    pub fn get_target_unit(self) -> crate::app::unit::Unit;

    #[method(name = "PlayEffect", args = 0)]
    pub fn play_effect(self) -> ();

    #[method(name = "HealHp", args = 2)]
    pub fn heal_hp(unit: crate::app::unit::Unit, heal: i32) -> ();

    #[method(name = "TryActiveSkill", args = 2)]
    pub fn try_active_skill(
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "HealHp", args = 0)]
    pub fn heal_hp_2(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ItemUse", args = 0)]
    pub fn item_use(self) -> ();

    #[method(name = "GainExp", args = 0)]
    pub fn gain_exp(self) -> ();

    #[method(name = "GetUseType", args = 0)]
    pub fn get_use_type(self) -> crate::app::itemdata::ItemData_UseTypes;

    #[method(name = "IsWeaponEnchant", args = 0)]
    pub fn is_weapon_enchant(self) -> bool;

    #[method(name = "WeaponEnchant", args = 0)]
    pub fn weapon_enchant(self) -> ();

    #[method(name = "ShowMessage", args = 0)]
    pub fn show_message(self) -> ();

    #[method(name = "TryExpend", args = 1)]
    pub fn try_expend(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "Expend", args = 0)]
    pub fn expend(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ClassChange", args = 0)]
    pub fn class_change(self) -> ();

    #[method(name = "ClassChangeRescue", args = 0)]
    pub fn class_change_rescue(self) -> ();

    #[method(name = "GetHealHp", args = 1)]
    pub fn get_heal_hp(self, target: crate::app::unit::Unit) -> i32;

    #[method(name = "CanHealHp", args = 1)]
    pub fn can_heal_hp(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "CanGain", args = 1)]
    pub fn can_gain(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "TryNextTarget", args = 0)]
    pub fn try_next_target(self) -> ();

    #[method(name = "TryGrow", args = 0)]
    pub fn try_grow(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-itemusesequence")]
impl ItemUseSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemUseSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IItemUseSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemusesequence/ItemUseSequence_EnchantedUnitList.md")))]
#[::unity2::class(namespace = "App", name = "ItemUseSequence.EnchantedUnitList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: unit :: Unit >)]
pub struct ItemUseSequence_EnchantedUnitList {}

#[cfg(feature = "app-itemusesequence")]
#[::unity2::methods]
impl ItemUseSequence_EnchantedUnitList {
    #[method(name = "Search", args = 0)]
    pub fn search(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemusesequence")]
impl ItemUseSequence_EnchantedUnitList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemUseSequence_EnchantedUnitList),
                ::core::stringify!(new),
            )
        });
        <Self as IItemUseSequence_EnchantedUnitListMethods>::ctor(this);
        this
    }
}
