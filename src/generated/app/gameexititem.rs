
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameexititem/GameExitItem.md")))]
#[::unity2::class(namespace = "App", name = "GameExitItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct GameExitItem {}

#[cfg(feature = "app-gameexititem")]
#[::unity2::methods]
impl GameExitItem {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameEnglish", args = 0)]
    pub fn get_name_english(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameexititem")]
impl GameExitItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameExitItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGameExitItemMethods>::ctor(this);
        this
    }
}
