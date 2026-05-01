
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmanager/ChallengeMapSelectManager.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeMapSelectManager")]
#[parent(crate::system::object::Object)]
pub struct ChallengeMapSelectManager {
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::challengemapselectmanager::ChallengeMapSelectManager_ReturnEventHandler,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::challengemapselectmenu::ChallengeMapSelectMenu,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
}

#[cfg(feature = "app-challengemapselectmanager")]
#[::unity2::methods]
impl ChallengeMapSelectManager {
    #[method(name = "Create", args = 4)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        default_challenge_data: crate::app::challengedata::ChallengeData,
        return_event_handler : crate :: app :: challengemapselectmanager :: ChallengeMapSelectManager_ReturnEventHandler,
    ) -> crate::app::challengemapselectmanager::ChallengeMapSelectManager;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        default_challenge_data: crate::app::challengedata::ChallengeData,
        return_event_handler : crate :: app :: challengemapselectmanager :: ChallengeMapSelectManager_ReturnEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 1)]
    pub fn on_select(self, challenge_data: crate::app::challengedata::ChallengeData) -> ();

    #[method(name = "OnDecide", args = 1)]
    pub fn on_decide(self, challenge_data: crate::app::challengedata::ChallengeData) -> ();

    #[method(name = "OnRequestClose", args = 0)]
    pub fn on_request_close(self) -> ();
}

#[cfg(feature = "app-challengemapselectmanager")]
impl ChallengeMapSelectManager {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::challengemapselectroot::ChallengeMapSelectRoot,
        default_challenge_data: crate::app::challengedata::ChallengeData,
        return_event_handler : crate :: app :: challengemapselectmanager :: ChallengeMapSelectManager_ReturnEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectManager),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectManagerMethods>::ctor(
            this,
            super_,
            root,
            default_challenge_data,
            return_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengemapselectmanager/ChallengeMapSelectManager_ReturnEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ChallengeMapSelectManager.ReturnEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ChallengeMapSelectManager_ReturnEventHandler {}

#[cfg(feature = "app-challengemapselectmanager")]
#[::unity2::methods]
impl ChallengeMapSelectManager_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        challenge_data: crate::app::challengedata::ChallengeData,
    ) -> ();
}

#[cfg(feature = "app-challengemapselectmanager")]
impl ChallengeMapSelectManager_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeMapSelectManager_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeMapSelectManager_ReturnEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
