
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmenu/ChallengeMapSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeMapSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeMapSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-challengemapselectmenu")]
#[::unity2::methods]
impl ChallengeMapSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, chapter_data: crate::app::challengedata::ChallengeData) -> ();
}

#[cfg(feature = "app-challengemapselectmenu")]
impl ChallengeMapSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmenu/ChallengeMapSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeMapSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ChallengeMapSelectMenu {}

#[cfg(feature = "app-challengemapselectmenu")]
#[::unity2::methods]
impl ChallengeMapSelectMenu {
    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        default_challenge_data: crate::app::challengedata::ChallengeData,
        select_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: challengemapselectmenu :: ChallengeMapSelectMenu_RequestCloseEventHandler,
    ) -> crate::app::challengemapselectmenu::ChallengeMapSelectMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-challengemapselectmenu")]
impl ChallengeMapSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmenu/ChallengeMapSelectMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ChallengeMapSelectMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeMapSelectMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-challengemapselectmenu")]
#[::unity2::methods]
impl ChallengeMapSelectMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-challengemapselectmenu")]
impl ChallengeMapSelectMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmenu/ChallengeMapSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeMapSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeMapSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-challengemapselectmenu")]
#[::unity2::methods]
impl ChallengeMapSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, chapter_data: crate::app::challengedata::ChallengeData) -> ();
}

#[cfg(feature = "app-challengemapselectmenu")]
impl ChallengeMapSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
