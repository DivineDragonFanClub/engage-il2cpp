
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellyesnodialognomenuitem/ShopSellYesNoDialogNoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellYesNoDialogNoMenuItem")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct ShopSellYesNoDialogNoMenuItem {}

#[cfg(feature = "app-shopsellyesnodialognomenuitem")]
#[::unity2::methods]
impl ShopSellYesNoDialogNoMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-shopsellyesnodialognomenuitem")]
impl ShopSellYesNoDialogNoMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellYesNoDialogNoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellYesNoDialogNoMenuItemMethods>::ctor(this);
        this
    }
}
