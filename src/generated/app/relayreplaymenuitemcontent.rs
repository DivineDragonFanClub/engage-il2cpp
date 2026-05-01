
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayreplaymenuitemcontent/RelayReplayMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RelayReplayMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RelayReplayMenuItemContent {
    #[rename(name = "m_UserNameText")]
    pub m_user_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BattleIcon")]
    pub m_battle_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PresentIcon")]
    pub m_present_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CheckIcon")]
    pub m_check_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-relayreplaymenuitemcontent")]
#[::unity2::methods]
impl RelayReplayMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayreplaymenuitemcontent")]
impl RelayReplayMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayReplayMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayReplayMenuItemContentMethods>::ctor(this);
        this
    }
}
