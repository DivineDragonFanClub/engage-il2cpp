
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/challengedifficultymenuitem/ChallengeDifficultyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ChallengeDifficultyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ChallengeDifficultyMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::challengedifficultymenu::ChallengeDifficultyMenu_DecideEventHandler,
}

#[cfg(feature = "app-challengedifficultymenuitem")]
#[::unity2::methods]
impl ChallengeDifficultyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-challengedifficultymenuitem")]
impl ChallengeDifficultyMenuItem {
    pub fn new(
        decide_event_handler : crate :: app :: challengedifficultymenu :: ChallengeDifficultyMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChallengeDifficultyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IChallengeDifficultyMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}
