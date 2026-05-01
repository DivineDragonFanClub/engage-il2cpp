
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondlevelselectmenuitemcontent/ArenaBondLevelSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondLevelSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ArenaBondLevelSelectMenuItemContent {
    #[rename(name = "m_LevelTitle")]
    pub m_level_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Cost")]
    pub m_cost: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CostIcon")]
    pub m_cost_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_TalkRoot")]
    pub m_talk_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TalkIcon")]
    pub m_talk_icon: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-arenabondlevelselectmenuitemcontent")]
#[::unity2::methods]
impl ArenaBondLevelSelectMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenabondlevelselectmenuitemcontent")]
impl ArenaBondLevelSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondLevelSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondLevelSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
