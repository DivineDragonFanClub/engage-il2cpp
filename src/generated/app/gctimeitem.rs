
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::paramitem::IParamItem;
use crate::app::paramitem::ParamItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gctimeitem/GCTimeItem.md")))]
#[::unity2::class(namespace = "App", name = "GCTimeItem")]
#[parent(crate::app::paramitem::ParamItem)]
pub struct GCTimeItem {}

#[cfg(feature = "app-gctimeitem")]
#[::unity2::methods]
impl GCTimeItem {
    #[method(name = "GetColumnName0", args = 0)]
    pub fn get_column_name0(self) -> ::unity2::Il2CppString;

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "GetColumnName1", args = 0)]
    pub fn get_column_name1(self) -> ::unity2::Il2CppString;

    #[method(name = "OnLeftRight", args = 2)]
    pub fn on_left_right(self, step: i32, is_trigger: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gctimeitem")]
impl GCTimeItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GCTimeItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGCTimeItemMethods>::ctor(this);
        this
    }
}
