
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::inventorypoolitemmenuitem::IInventoryPoolItemMenuItem;
use crate::app::inventorypoolitemmenuitem::InventoryPoolItemMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorypoolitemmenublankitem/InventoryPoolItemMenuBlankItem.md")))]
#[::unity2::class(namespace = "App", name = "InventoryPoolItemMenuBlankItem")]
#[parent(crate::app::inventorypoolitemmenuitem::InventoryPoolItemMenuItem)]
pub struct InventoryPoolItemMenuBlankItem {}

#[cfg(feature = "app-inventorypoolitemmenublankitem")]
#[::unity2::methods]
impl InventoryPoolItemMenuBlankItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetMenuItemKind", args = 0)]
    pub fn get_menu_item_kind(
        self,
    ) -> crate::app::inventorypoolitemmenuitem::InventoryPoolItemMenuItem_PoolItemKind;

    #[method(name = "IsVisibleItemIconOnBlank", args = 0)]
    pub fn is_visible_item_icon_on_blank(self) -> bool;
}

#[cfg(feature = "app-inventorypoolitemmenublankitem")]
impl InventoryPoolItemMenuBlankItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryPoolItemMenuBlankItem),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryPoolItemMenuBlankItemMethods>::ctor(this);
        this
    }
}
