
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmenuitem/ChallengeMapSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeMapSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ChallengeMapSelectMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::challengemapselectmenu::ChallengeMapSelectMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::challengemapselectmenu::ChallengeMapSelectMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::challengemapselectmenu::ChallengeMapSelectMenu_RequestCloseEventHandler,
    #[rename(name = "m_ChallengeData")]
    pub m_challenge_data: crate::app::challengedata::ChallengeData,
}

#[cfg(feature = "app-challengemapselectmenuitem")]
#[::unity2::methods]
impl ChallengeMapSelectMenuItem {
    #[method(name = "IsNew", args = 0)]
    pub fn is_new(self) -> bool;

    #[method(name = "SawNewAccess", args = 0)]
    pub fn saw_new_access(self) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        challenge_data: crate::app::challengedata::ChallengeData,
        select_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-challengemapselectmenuitem")]
impl ChallengeMapSelectMenuItem {
    pub fn new(
        challenge_data: crate::app::challengedata::ChallengeData,
        select_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectMenuItemMethods>::ctor(
            this,
            challenge_data,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}
