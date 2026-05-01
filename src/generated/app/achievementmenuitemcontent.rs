
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievementmenuitemcontent/AchievementMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "AchievementMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct AchievementMenuItemContent {
    #[rename(name = "m_countRoot")]
    pub m_count_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_countValue")]
    pub m_count_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_clearRoot")]
    pub m_clear_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_bonusRoot")]
    pub m_bonus_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_bonusValue")]
    pub m_bonus_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_completeRoot")]
    pub m_complete_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_countText")]
    pub m_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_clearText")]
    pub m_clear_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_bonusText")]
    pub m_bonus_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_completeText")]
    pub m_complete_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-achievementmenuitemcontent")]
#[::unity2::methods]
impl AchievementMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "SynchronizeClearAnimationTime", args = 0)]
    pub fn synchronize_clear_animation_time(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-achievementmenuitemcontent")]
impl AchievementMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AchievementMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAchievementMenuItemContentMethods>::ctor(this);
        this
    }
}
