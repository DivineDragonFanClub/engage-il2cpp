
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardvisualbasemenuitem::IProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualtextdecomenuitem/ProfileCardVisualTextDecoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualTextDecoMenuItem")]
#[parent(crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem)]
pub struct ProfileCardVisualTextDecoMenuItem {}

#[cfg(feature = "app-profilecardvisualtextdecomenuitem")]
#[::unity2::methods]
impl ProfileCardVisualTextDecoMenuItem {
    #[method(name = "get_m_TextDecoData", args = 0)]
    pub fn get_m_text_deco_data(
        self,
    ) -> crate::app::profilecardtextdecodata::ProfileCardTextDecoData;

    #[method(name = "set_m_TextDecoData", args = 1)]
    pub fn set_m_text_deco_data(
        self,
        value: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text_deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
        initial_select: bool,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();
}

#[cfg(feature = "app-profilecardvisualtextdecomenuitem")]
impl ProfileCardVisualTextDecoMenuItem {
    pub fn new(
        text_deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualTextDecoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualTextDecoMenuItemMethods>::ctor(
            this,
            text_deco_data,
            initial_select,
        );
        this
    }
}
