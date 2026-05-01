
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiiboaccessorymenunoneitem/AmiiboAccessoryMenuNoneItem.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboAccessoryMenuNoneItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AmiiboAccessoryMenuNoneItem {}

#[cfg(feature = "app-amiiboaccessorymenunoneitem")]
#[::unity2::methods]
impl AmiiboAccessoryMenuNoneItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-amiiboaccessorymenunoneitem")]
impl AmiiboAccessoryMenuNoneItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboAccessoryMenuNoneItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboAccessoryMenuNoneItemMethods>::ctor(this);
        this
    }
}
