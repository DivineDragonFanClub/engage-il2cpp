
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenuitem/ProfileCardAlbumListMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardAlbumListMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardAlbumListMenuItem {
    #[rename(name = "m_Profile")]
    pub m_profile: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardalbumlistmenuitem")]
#[::unity2::methods]
impl ProfileCardAlbumListMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        profile: crate::app::profilecard::ProfileCard,
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardalbumlistmenuitem")]
impl ProfileCardAlbumListMenuItem {
    pub fn new(
        profile: crate::app::profilecard::ProfileCard,
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenuItemMethods>::ctor(this, profile, decide_event_handler);
        this
    }
}
