
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardvisualbasemenuitem::IProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualtextcolormenuitem/ProfileCardVisualTextColorMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualTextColorMenuItem")]
#[parent(crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem)]
pub struct ProfileCardVisualTextColorMenuItem {}

#[cfg(feature = "app-profilecardvisualtextcolormenuitem")]
#[::unity2::methods]
impl ProfileCardVisualTextColorMenuItem {
    #[method(name = "get_m_TextColorData", args = 0)]
    pub fn get_m_text_color_data(
        self,
    ) -> crate::app::profilecardtextcolordata::ProfileCardTextColorData;

    #[method(name = "set_m_TextColorData", args = 1)]
    pub fn set_m_text_color_data(
        self,
        value: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text_color_data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
        initial_select: bool,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();
}

#[cfg(feature = "app-profilecardvisualtextcolormenuitem")]
impl ProfileCardVisualTextColorMenuItem {
    pub fn new(
        text_color_data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualTextColorMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualTextColorMenuItemMethods>::ctor(
            this,
            text_color_data,
            initial_select,
        );
        this
    }
}
