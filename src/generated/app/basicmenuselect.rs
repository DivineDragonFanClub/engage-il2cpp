
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicmenuselect/BasicMenuSelect.md")))]
#[::unity2::class(namespace = "App", name = "BasicMenuSelect")]
#[parent(crate::system::object::Object)]
pub struct BasicMenuSelect {}

#[cfg(feature = "app-basicmenuselect")]
#[::unity2::methods]
impl BasicMenuSelect {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, other: crate::app::basicmenuselect::BasicMenuSelect) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Duplicate", args = 0)]
    pub fn duplicate(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_Scroll", args = 0)]
    pub fn get_scroll(self) -> i32;

    #[method(name = "set_Scroll", args = 1)]
    pub fn set_scroll(self, value: i32) -> ();
}

#[cfg(feature = "app-basicmenuselect")]
impl BasicMenuSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicMenuSelectMethods>::ctor(this);
        this
    }

    pub fn new_2(other: crate::app::basicmenuselect::BasicMenuSelect) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuSelect),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBasicMenuSelectMethods>::ctor_2(this, other);
        this
    }
}
