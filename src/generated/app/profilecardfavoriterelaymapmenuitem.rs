
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardfavoriterelaymapmenuitem/ProfileCardFavoriteRelayMapMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFavoriteRelayMapMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardFavoriteRelayMapMenuItem {}

#[cfg(feature = "app-profilecardfavoriterelaymapmenuitem")]
#[::unity2::methods]
impl ProfileCardFavoriteRelayMapMenuItem {
    #[method(name = "get_m_FavoriteMapData", args = 0)]
    pub fn get_m_favorite_map_data(
        self,
    ) -> crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData;

    #[method(name = "set_m_FavoriteMapData", args = 1)]
    pub fn set_m_favorite_map_data(
        self,
        value: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> ();

    #[method(name = "get_m_Decided", args = 0)]
    pub fn get_m_decided(self) -> bool;

    #[method(name = "set_m_Decided", args = 1)]
    pub fn set_m_decided(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        favorite_map_data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
        initial_select: bool,
    ) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "UpdateFixedCursor", args = 0)]
    pub fn update_fixed_cursor(self) -> ();

    #[method(name = "UpdateNewIcon", args = 0)]
    pub fn update_new_icon(self) -> ();

    #[method(name = "SetDecided", args = 1)]
    pub fn set_decided(self, decided: bool) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardfavoriterelaymapmenuitem")]
impl ProfileCardFavoriteRelayMapMenuItem {
    pub fn new(
        favorite_map_data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFavoriteRelayMapMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFavoriteRelayMapMenuItemMethods>::ctor(
            this,
            favorite_map_data,
            initial_select,
        );
        this
    }
}
