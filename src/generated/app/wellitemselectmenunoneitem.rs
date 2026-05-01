
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::wellitemselectmenuitem::IWellItemSelectMenuItem;
use crate::app::wellitemselectmenuitem::WellItemSelectMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellitemselectmenunoneitem/WellItemSelectMenuNoneItem.md")))]
#[::unity2::class(namespace = "App", name = "WellItemSelectMenuNoneItem")]
#[parent(crate::app::wellitemselectmenuitem::WellItemSelectMenuItem)]
pub struct WellItemSelectMenuNoneItem {}

#[cfg(feature = "app-wellitemselectmenunoneitem")]
#[::unity2::methods]
impl WellItemSelectMenuNoneItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetBlankText", args = 0)]
    pub fn get_blank_text(self) -> ::unity2::Il2CppString;

    #[method(name = "IsVisibleItemIconOnBlank", args = 0)]
    pub fn is_visible_item_icon_on_blank(self) -> bool;
}

#[cfg(feature = "app-wellitemselectmenunoneitem")]
impl WellItemSelectMenuNoneItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellItemSelectMenuNoneItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWellItemSelectMenuNoneItemMethods>::ctor(this);
        this
    }
}
