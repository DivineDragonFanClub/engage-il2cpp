
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::profilecardcommentlistmenuitem::IProfileCardCommentListMenuItem;
use crate::app::profilecardcommentlistmenuitem::ProfileCardCommentListMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistdoemptymenuitem/ProfileCardCommentListDoEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentListDoEmptyMenuItem")]
#[parent(crate::app::profilecardcommentlistmenuitem::ProfileCardCommentListMenuItem)]
pub struct ProfileCardCommentListDoEmptyMenuItem {}

#[cfg(feature = "app-profilecardcommentlistdoemptymenuitem")]
#[::unity2::methods]
impl ProfileCardCommentListDoEmptyMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        initial_select: bool,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
    ) -> ();
}

#[cfg(feature = "app-profilecardcommentlistdoemptymenuitem")]
impl ProfileCardCommentListDoEmptyMenuItem {
    pub fn new(
        initial_select: bool,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListDoEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListDoEmptyMenuItemMethods>::ctor(
            this,
            initial_select,
            select_event_handler,
        );
        this
    }
}
