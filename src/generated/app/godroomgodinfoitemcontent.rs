
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomgodinfoitemcontent/GodRoomGodInfoItemContent.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomGodInfoItemContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GodRoomGodInfoItemContent {
    #[rename(name = "m_GodName")]
    pub m_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Level")]
    pub m_level: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LevelTitle")]
    pub m_level_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_LevelGauge")]
    pub m_level_gauge: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LevelGaugeImage")]
    pub m_level_gauge_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-godroomgodinfoitemcontent")]
#[::unity2::methods]
impl GodRoomGodInfoItemContent {
    #[method(name = "SetData", args = 2)]
    pub fn set_data(self, unit: crate::app::unit::Unit, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godroomgodinfoitemcontent")]
impl GodRoomGodInfoItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomGodInfoItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomGodInfoItemContentMethods>::ctor(this);
        this
    }
}
