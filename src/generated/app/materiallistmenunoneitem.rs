
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materiallistmenunoneitem/MaterialListMenuNoneItem.md")))]
#[::unity2::class(namespace = "App", name = "MaterialListMenuNoneItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MaterialListMenuNoneItem {}

#[cfg(feature = "app-materiallistmenunoneitem")]
#[::unity2::methods]
impl MaterialListMenuNoneItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-materiallistmenunoneitem")]
impl MaterialListMenuNoneItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialListMenuNoneItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialListMenuNoneItemMethods>::ctor(this);
        this
    }
}
