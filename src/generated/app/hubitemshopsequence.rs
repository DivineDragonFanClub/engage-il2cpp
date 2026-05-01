
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubitemshopsequence/HubItemShopSequence.md")))]
#[::unity2::class(namespace = "App", name = "HubItemShopSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubItemShopSequence {
    #[rename(name = "m_ShopMenuResult")]
    pub m_shop_menu_result: crate::app::itemshoptopmenu::ItemShopTopMenu_Result2,
    #[rename(name = "m_ShopUnitSelectMenuResult")]
    pub m_shop_unit_select_menu_result: crate::app::basicmenu::BasicMenu_Result,
    #[rename(name = "m_UnitSelectRoot")]
    pub m_unit_select_root: crate::app::shopunitselectroot::ShopUnitSelectRoot,
    #[rename(name = "m_ItemShopBuyRoot")]
    pub m_item_shop_buy_root: crate::app::itemshopbuyroot::ItemShopBuyRoot,
    #[rename(name = "m_ItemShopSellRoot")]
    pub m_item_shop_sell_root: crate::app::shopsellroot::ShopSellRoot,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitSelectMenuScrollIndex")]
    pub m_unit_select_menu_scroll_index: i32,
}

#[cfg(feature = "app-hubitemshopsequence")]
#[::unity2::methods]
impl HubItemShopSequence {
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

    #[method(name = "CreateItemShopTopMenu", args = 0)]
    pub fn create_item_shop_top_menu(self) -> ();

    #[method(name = "CreateShopUnitSelectMenu", args = 0)]
    pub fn create_shop_unit_select_menu(self) -> ();

    #[method(name = "DestroyShopUnitSelectMenu", args = 0)]
    pub fn destroy_shop_unit_select_menu(self) -> ();

    #[method(name = "CreateItemShopBuyMenu", args = 0)]
    pub fn create_item_shop_buy_menu(self) -> ();

    #[method(name = "DestroyItemShopBuyMenu", args = 0)]
    pub fn destroy_item_shop_buy_menu(self) -> ();

    #[method(name = "CreateItemShopSellMenu", args = 0)]
    pub fn create_item_shop_sell_menu(self) -> ();

    #[method(name = "DestroyItemShopSellMenu", args = 0)]
    pub fn destroy_item_shop_sell_menu(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();
}

#[cfg(feature = "app-hubitemshopsequence")]
impl HubItemShopSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubItemShopSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IHubItemShopSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubitemshopsequence/HubItemShopSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubItemShopSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubItemShopSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubItemShopSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubItemShopSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubItemShopSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }

    pub fn unit_select_to_buy() -> Self {
        Self { value: 2 }
    }

    pub fn buy() -> Self {
        Self { value: 3 }
    }

    pub fn unit_select_to_sell() -> Self {
        Self { value: 4 }
    }

    pub fn sell() -> Self {
        Self { value: 5 }
    }

    pub fn end() -> Self {
        Self { value: 6 }
    }
}
