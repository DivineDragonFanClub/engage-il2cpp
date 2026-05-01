
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugeditmenuitem/DebugEditMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "DebugEditMenuItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct DebugEditMenuItem {}

#[cfg(feature = "app-debugeditmenuitem")]
#[::unity2::methods]
impl DebugEditMenuItem {
    #[method(name = "TickValue", args = 0)]
    pub fn tick_value(self) -> bool;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "IsSkip", args = 1)]
    pub fn is_skip(self, v: i32) -> bool;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "get_MinValue", args = 0)]
    pub fn get_min_value(self) -> i32;

    #[method(name = "get_MaxValue", args = 0)]
    pub fn get_max_value(self) -> i32;

    #[method(name = "get_Step", args = 0)]
    pub fn get_step(self) -> i32;

    #[method(name = "get_Fast", args = 0)]
    pub fn get_fast(self) -> i32;

    #[method(name = "get_IsLoop", args = 0)]
    pub fn get_is_loop(self) -> bool;

    #[method(name = "CanIncrement", args = 1)]
    pub fn can_increment(self, value: i32) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugeditmenuitem")]
impl DebugEditMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugEditMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugEditMenuItemMethods>::ctor(this);
        this
    }
}
