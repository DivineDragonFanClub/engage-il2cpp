
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardvisualbasemenuitem::IProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualbasemenuitem::ProfileCardVisualBaseMenuItem;
use crate::app::profilecardvisualframemenuitem::IProfileCardVisualFrameMenuItem;
use crate::app::profilecardvisualframemenuitem::ProfileCardVisualFrameMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualframeemptymenuitem/ProfileCardVisualFrameEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualFrameEmptyMenuItem")]
#[parent(crate::app::profilecardvisualframemenuitem::ProfileCardVisualFrameMenuItem)]
pub struct ProfileCardVisualFrameEmptyMenuItem {}

#[cfg(feature = "app-profilecardvisualframeemptymenuitem")]
#[::unity2::methods]
impl ProfileCardVisualFrameEmptyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, initial_select: bool) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "IsNewArrival", args = 0)]
    pub fn is_new_arrival(self) -> bool;

    #[method(name = "SetAlreadyRead", args = 0)]
    pub fn set_already_read(self) -> ();
}

#[cfg(feature = "app-profilecardvisualframeemptymenuitem")]
impl ProfileCardVisualFrameEmptyMenuItem {
    pub fn new(initial_select: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualFrameEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualFrameEmptyMenuItemMethods>::ctor(this, initial_select);
        this
    }
}
