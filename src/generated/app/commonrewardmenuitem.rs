
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewardmenuitem/CommonRewardMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "CommonRewardMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct CommonRewardMenuItem {}

#[cfg(feature = "app-commonrewardmenuitem")]
#[::unity2::methods]
impl CommonRewardMenuItem {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_UnitLevelOld", args = 0)]
    pub fn get_unit_level_old(self) -> i32;

    #[method(name = "set_UnitLevelOld", args = 1)]
    pub fn set_unit_level_old(self, value: i32) -> ();

    #[method(name = "get_IsAddInEnd", args = 0)]
    pub fn get_is_add_in_end(self) -> bool;

    #[method(name = "set_IsAddInEnd", args = 1)]
    pub fn set_is_add_in_end(self, value: bool) -> ();

    #[method(name = "get_AddExp", args = 0)]
    pub fn get_add_exp(self) -> i32;

    #[method(name = "set_AddExp", args = 1)]
    pub fn set_add_exp(self, value: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, add_exp: i32) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-commonrewardmenuitem")]
impl CommonRewardMenuItem {
    pub fn new(unit: crate::app::unit::Unit, add_exp: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRewardMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRewardMenuItemMethods>::ctor(this, unit, add_exp);
        this
    }
}
