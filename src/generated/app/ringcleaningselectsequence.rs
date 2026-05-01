
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningselectsequence/RingCleaningSelectSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RingCleaningSelectSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RingCleaningSelectSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RingCleaningSelectSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RingCleaningSelectSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RingCleaningSelectSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn ring_select() -> Self {
        Self { value: 1 }
    }

    pub fn cleaner_select() -> Self {
        Self { value: 2 }
    }

    pub fn ring_cleaning() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningselectsequence/RingCleaningSelectSequence.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningSelectSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RingCleaningSelectSequence {
    #[rename(name = "m_MenuItemResult")]
    pub m_menu_item_result: crate::app::basicmenu::BasicMenu_Result,
    #[rename(name = "m_GodSelectRoot")]
    pub m_god_select_root: crate::app::godselectroot::GodSelectRoot,
    #[rename(name = "m_SelectUnit")]
    pub m_select_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-ringcleaningselectsequence")]
#[::unity2::methods]
impl RingCleaningSelectSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateGodSelectMenu", args = 0)]
    pub fn create_god_select_menu(self) -> ();

    #[method(name = "DestroyGodSelectMenu", args = 0)]
    pub fn destroy_god_select_menu(self) -> ();

    #[method(name = "CreateUnitSelectMenu", args = 0)]
    pub fn create_unit_select_menu(self) -> ();

    #[method(name = "DestroyUnitSelectMenu", args = 0)]
    pub fn destroy_unit_select_menu(self) -> ();

    #[method(name = "StartRingCleaning", args = 0)]
    pub fn start_ring_cleaning(self) -> ();

    #[method(name = "EndRingCleaning", args = 0)]
    pub fn end_ring_cleaning(self) -> ();

    #[method(name = "IsDecided", args = 0)]
    pub fn is_decided(self) -> bool;

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();
}

#[cfg(feature = "app-ringcleaningselectsequence")]
impl RingCleaningSelectSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningSelectSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningSelectSequenceMethods>::ctor(this);
        this
    }
}
