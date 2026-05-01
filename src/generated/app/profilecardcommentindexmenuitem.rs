
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentindexmenuitem/ProfileCardCommentIndexMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentIndexMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardCommentIndexMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardcommentindexmenu::ProfileCardCommentIndexMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardcommentindexmenuitem")]
#[::unity2::methods]
impl ProfileCardCommentIndexMenuItem {
    #[method(name = "get_m_Mid", args = 0)]
    pub fn get_m_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_m_Mid", args = 1)]
    pub fn set_m_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        mid: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: profilecardcommentindexmenu :: ProfileCardCommentIndexMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "SetMessage", args = 1)]
    pub fn set_message(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardcommentindexmenuitem")]
impl ProfileCardCommentIndexMenuItem {
    pub fn new(
        mid: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: profilecardcommentindexmenu :: ProfileCardCommentIndexMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentIndexMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentIndexMenuItemMethods>::ctor(this, mid, decide_event_handler);
        this
    }
}
