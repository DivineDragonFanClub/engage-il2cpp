
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardvisualbasemenuitem::IProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualframemenuitem/ProfileCardVisualFrameMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualFrameMenuItem")]
#[parent(crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem)]
pub struct ProfileCardVisualFrameMenuItem {}

#[cfg(feature = "app-profilecardvisualframemenuitem")]
#[::unity2::methods]
impl ProfileCardVisualFrameMenuItem {
    #[method(name = "get_m_FrameData", args = 0)]
    pub fn get_m_frame_data(self) -> crate::app::profilecardframedata::ProfileCardFrameData;

    #[method(name = "set_m_FrameData", args = 1)]
    pub fn set_m_frame_data(
        self,
        value: crate::app::profilecardframedata::ProfileCardFrameData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
        initial_select: bool,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();
}

#[cfg(feature = "app-profilecardvisualframemenuitem")]
impl ProfileCardVisualFrameMenuItem {
    pub fn new(
        frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
        initial_select: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualFrameMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualFrameMenuItemMethods>::ctor(this, frame_data, initial_select);
        this
    }
}
