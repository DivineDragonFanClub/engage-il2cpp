
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtitlemenuitem/ProfileCardTitleMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTitleMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTitleMenuItem {}

#[cfg(feature = "app-profilecardtitlemenuitem")]
#[::unity2::methods]
impl ProfileCardTitleMenuItem {
    #[method(name = "get_m_TitleData", args = 0)]
    pub fn get_m_title_data(self) -> crate::app::profilecardtitledata::ProfileCardTitleData;

    #[method(name = "set_m_TitleData", args = 1)]
    pub fn set_m_title_data(
        self,
        value: crate::app::profilecardtitledata::ProfileCardTitleData,
    ) -> ();

    #[method(name = "get_m_Decided", args = 0)]
    pub fn get_m_decided(self) -> bool;

    #[method(name = "set_m_Decided", args = 1)]
    pub fn set_m_decided(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        title_data: crate::app::profilecardtitledata::ProfileCardTitleData,
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

#[cfg(feature = "app-profilecardtitlemenuitem")]
impl ProfileCardTitleMenuItem {
    pub fn new(
        title_data: crate::app::profilecardtitledata::ProfileCardTitleData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTitleMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTitleMenuItemMethods>::ctor(this, title_data, initial_select);
        this
    }
}
