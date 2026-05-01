
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardvisualbasemenuitem::IProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualcharacterstampmenuitem/ProfileCardVisualCharacterStampMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualCharacterStampMenuItem")]
#[parent(crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem)]
pub struct ProfileCardVisualCharacterStampMenuItem {}

#[cfg(feature = "app-profilecardvisualcharacterstampmenuitem")]
#[::unity2::methods]
impl ProfileCardVisualCharacterStampMenuItem {
    #[method(name = "get_m_CharacterStampData", args = 0)]
    pub fn get_m_character_stamp_data(
        self,
    ) -> crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData;

    #[method(name = "set_m_CharacterStampData", args = 1)]
    pub fn set_m_character_stamp_data(
        self,
        value: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        character_stamp_data : crate :: app :: profilecardcharacterstampdata :: ProfileCardCharacterStampData,
        initial_select: bool,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();
}

#[cfg(feature = "app-profilecardvisualcharacterstampmenuitem")]
impl ProfileCardVisualCharacterStampMenuItem {
    pub fn new(
        character_stamp_data : crate :: app :: profilecardcharacterstampdata :: ProfileCardCharacterStampData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualCharacterStampMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualCharacterStampMenuItemMethods>::ctor(
            this,
            character_stamp_data,
            initial_select,
        );
        this
    }
}
