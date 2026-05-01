
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardfavoriterelaymapmenuitem::IProfileCardFavoriteRelayMapMenuItem;
use crate::app::profilecardfavoriterelaymapmenuitem::ProfileCardFavoriteRelayMapMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardfavoriterelaymapemptymenuitem/ProfileCardFavoriteRelayMapEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFavoriteRelayMapEmptyMenuItem")]
#[parent(crate::app::profilecardfavoriterelaymapmenuitem::ProfileCardFavoriteRelayMapMenuItem)]
pub struct ProfileCardFavoriteRelayMapEmptyMenuItem {}

#[cfg(feature = "app-profilecardfavoriterelaymapemptymenuitem")]
#[::unity2::methods]
impl ProfileCardFavoriteRelayMapEmptyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, initial_select: bool) -> ();
}

#[cfg(feature = "app-profilecardfavoriterelaymapemptymenuitem")]
impl ProfileCardFavoriteRelayMapEmptyMenuItem {
    pub fn new(initial_select: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFavoriteRelayMapEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFavoriteRelayMapEmptyMenuItemMethods>::ctor(this, initial_select);
        this
    }
}
