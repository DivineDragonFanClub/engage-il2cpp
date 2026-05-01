
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengedifficultymenu/ChallengeDifficultyMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ChallengeDifficultyMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeDifficultyMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-challengedifficultymenu")]
#[::unity2::methods]
impl ChallengeDifficultyMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, difficulty: i32) -> ();
}

#[cfg(feature = "app-challengedifficultymenu")]
impl ChallengeDifficultyMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeDifficultyMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeDifficultyMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengedifficultymenu/ChallengeDifficultyMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeDifficultyMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeDifficultyMenu_DecideEventHandler {}

#[cfg(feature = "app-challengedifficultymenu")]
#[::unity2::methods]
impl ChallengeDifficultyMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, difficulty: i32) -> ();
}

#[cfg(feature = "app-challengedifficultymenu")]
impl ChallengeDifficultyMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeDifficultyMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeDifficultyMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengedifficultymenu/ChallengeDifficultyMenu.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeDifficultyMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ChallengeDifficultyMenu {
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::challengedifficultymenu::ChallengeDifficultyMenu_RequestCloseEventHandler,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
    #[rename(name = "m_TextDifficulty")]
    pub m_text_difficulty: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-challengedifficultymenu")]
#[::unity2::methods]
impl ChallengeDifficultyMenu {
    #[method(name = "get_Difficulty", args = 0)]
    pub fn get_difficulty(self) -> i32;

    #[method(name = "set_Difficulty", args = 1)]
    pub fn set_difficulty(self, value: i32) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        decide_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_RequestCloseEventHandler,
    ) -> crate::app::challengedifficultymenu::ChallengeDifficultyMenu;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        request_close_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-challengedifficultymenu")]
impl ChallengeDifficultyMenu {
    pub fn new(
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        request_close_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeDifficultyMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeDifficultyMenuMethods>::ctor(
            this,
            root,
            menu_item_list,
            menu_content,
            request_close_event_handler,
        );
        this
    }
}
