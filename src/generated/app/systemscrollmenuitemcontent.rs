
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemscrollmenuitemcontent/SystemScrollMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "SystemScrollMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct SystemScrollMenuItemContent {
    #[rename(name = "m_Partner")]
    pub m_partner: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Help")]
    pub m_help: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-systemscrollmenuitemcontent")]
#[::unity2::methods]
impl SystemScrollMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "SetHelpText", args = 2)]
    pub fn set_help_text(
        self,
        help: ::unity2::Il2CppString,
        c: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetHelpActive", args = 2)]
    pub fn set_help_active(self, b_active: bool, c: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetIcon", args = 1)]
    pub fn set_icon(self, spot: crate::app::gmapspot::GmapSpot) -> ();

    #[method(name = "GetEncountIcon", args = 1)]
    pub fn get_encount_icon(
        self,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemscrollmenuitemcontent")]
impl SystemScrollMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemScrollMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemScrollMenuItemContentMethods>::ctor(this);
        this
    }
}
