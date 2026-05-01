
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::app::mainmenusequence::IMainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent;
use crate::app::mainmenusequence::MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/growmodeselectmenucontent/GrowModeSelectMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "GrowModeSelectMenuContent")]
#[parent(
    crate::app::mainmenusequence::MainMenuSequence_GrowModeSelectMenuSequence_Menu_MenuContent
)]
pub struct GrowModeSelectMenuContent {}

#[cfg(feature = "app-growmodeselectmenucontent")]
#[::unity2::methods]
impl GrowModeSelectMenuContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-growmodeselectmenucontent")]
impl GrowModeSelectMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GrowModeSelectMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGrowModeSelectMenuContentMethods>::ctor(this);
        this
    }
}
