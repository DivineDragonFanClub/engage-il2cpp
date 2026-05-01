
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsequnece/ShopSequnece.md")))]
#[::unity2::class(namespace = "App", name = "ShopSequnece")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ShopSequnece {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-shopsequnece")]
#[::unity2::methods]
impl ShopSequnece {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CreateMenu", args = 0)]
    pub fn create_menu(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        name: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
    ) -> ();
}

#[cfg(feature = "app-shopsequnece")]
impl ShopSequnece {
    pub fn new(name: ::unity2::Il2CppString, unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSequnece),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSequneceMethods>::ctor(this, name, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsequnece/ShopSequnece_ParamItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSequnece.ParamItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct ShopSequnece_ParamItem {}

#[cfg(feature = "app-shopsequnece")]
#[::unity2::methods]
impl ShopSequnece_ParamItem {
    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnWidth2", args = 0)]
    pub fn get_column_width2(self) -> f32;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnAlign2", args = 0)]
    pub fn get_column_align2(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopsequnece")]
impl ShopSequnece_ParamItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSequnece_ParamItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSequnece_ParamItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsequnece/ShopSequnece_GoldItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSequnece.GoldItem")]
#[parent(crate::app::shopsequnece::ShopSequnece_ParamItem)]
pub struct ShopSequnece_GoldItem {}

#[cfg(feature = "app-shopsequnece")]
#[::unity2::methods]
impl ShopSequnece_GoldItem {
    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopsequnece")]
impl ShopSequnece_GoldItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSequnece_GoldItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSequnece_GoldItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsequnece/ShopSequnece_StockItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSequnece.StockItem")]
#[parent(crate::app::shopsequnece::ShopSequnece_ParamItem)]
pub struct ShopSequnece_StockItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-shopsequnece")]
#[::unity2::methods]
impl ShopSequnece_StockItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-shopsequnece")]
impl ShopSequnece_StockItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSequnece_StockItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSequnece_StockItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsequnece/ShopSequnece_ShopItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSequnece.ShopItem")]
#[parent(crate::app::shopsequnece::ShopSequnece_ParamItem)]
pub struct ShopSequnece_ShopItem {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::shopdata::ShopData,
    #[rename(name = "m_Item")]
    pub m_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-shopsequnece")]
#[::unity2::methods]
impl ShopSequnece_ShopItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, data: crate::app::shopdata::ShopData, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "GetColumnName2", args = 0)]
    pub fn get_column_name2(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-shopsequnece")]
impl ShopSequnece_ShopItem {
    pub fn new(data: crate::app::shopdata::ShopData, unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSequnece_ShopItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSequnece_ShopItemMethods>::ctor(this, data, unit);
        this
    }
}
