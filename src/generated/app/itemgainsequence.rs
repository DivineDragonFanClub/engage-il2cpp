
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemgainsequence/ItemGainSequence.md")))]
#[::unity2::class(namespace = "App", name = "ItemGainSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ItemGainSequence {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_ItemData")]
    pub m_item_data: crate::app::itemdata::ItemData,
    #[rename(name = "m_Label")]
    pub m_label: ::unity2::Il2CppString,
    #[rename(name = "m_Count")]
    pub m_count: i32,
}

#[cfg(feature = "app-itemgainsequence")]
#[::unity2::methods]
impl ItemGainSequence {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        unit_item: crate::app::unititem::UnitItem,
        unit: crate::app::unit::Unit,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> ();

    #[method(name = "Gain", args = 0)]
    pub fn gain(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "Tutorial", args = 0)]
    pub fn tutorial(self) -> ();

    #[method(name = "IsPlayer", args = 1)]
    pub fn is_player(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        item: crate::app::itemdata::ItemData,
        count: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        item: crate::app::itemdata::ItemData,
        unit: crate::app::unit::Unit,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind_3(
        super_: crate::app::procinst::ProcInst,
        unit_item: crate::app::unititem::UnitItem,
        unit: crate::app::unit::Unit,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> ();
}

#[cfg(feature = "app-itemgainsequence")]
impl ItemGainSequence {
    pub fn new(
        unit_item: crate::app::unititem::UnitItem,
        unit: crate::app::unit::Unit,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemGainSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IItemGainSequenceMethods>::ctor(this, unit_item, unit, label, count);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemgainsequence/ItemGainSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ItemGainSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ItemGainSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ItemGainSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ItemGainSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ItemGainSequence_Label {
    pub fn gain() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}
