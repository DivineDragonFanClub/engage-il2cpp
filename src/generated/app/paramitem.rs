
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/paramitem/ParamItem.md")))]
#[::unity2::class(namespace = "App", name = "ParamItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct ParamItem {}

#[cfg(feature = "app-paramitem")]
#[::unity2::methods]
impl ParamItem {
    #[method(name = "GetColumnCount", args = 0)]
    pub fn get_column_count(self) -> i32;

    #[method(name = "GetColumnWidth0", args = 0)]
    pub fn get_column_width0(self) -> f32;

    #[method(name = "GetColumnWidth1", args = 0)]
    pub fn get_column_width1(self) -> f32;

    #[method(name = "GetColumnAlign1", args = 0)]
    pub fn get_column_align1(self) -> crate::app::menuitem::MenuItem_Align;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-paramitem")]
impl ParamItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ParamItem),
                ::core::stringify!(new),
            )
        });
        <Self as IParamItemMethods>::ctor(this);
        this
    }
}
