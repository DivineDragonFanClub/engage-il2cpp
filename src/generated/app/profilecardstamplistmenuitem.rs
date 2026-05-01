
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenuitem/ProfileCardStampListMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampListMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardStampListMenuItem {
    #[rename(name = "m_ProfileCardStampData")]
    pub m_profile_card_stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardstamplistmenu::ProfileCardStampListMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardstamplistmenuitem")]
#[::unity2::methods]
impl ProfileCardStampListMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        profile_card_stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        initial_select: bool,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "UpdateNewIcon", args = 0)]
    pub fn update_new_icon(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardstamplistmenuitem")]
impl ProfileCardStampListMenuItem {
    pub fn new(
        profile_card_stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        initial_select: bool,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenuItemMethods>::ctor(
            this,
            profile_card_stamp_data,
            initial_select,
            decide_event_handler,
        );
        this
    }
}
