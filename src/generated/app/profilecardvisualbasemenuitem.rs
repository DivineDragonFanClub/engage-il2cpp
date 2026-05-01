
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualbasemenuitem/ProfileCardVisualBaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualBaseMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardVisualBaseMenuItem {
    #[rename(name = "m_Decided")]
    pub m_decided: bool,
}

#[cfg(feature = "app-profilecardvisualbasemenuitem")]
#[::unity2::methods]
impl ProfileCardVisualBaseMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, initial_select: bool) -> ();

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

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();

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

#[cfg(feature = "app-profilecardvisualbasemenuitem")]
impl ProfileCardVisualBaseMenuItem {
    pub fn new(initial_select: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualBaseMenuItemMethods>::ctor(this, initial_select);
        this
    }
}
