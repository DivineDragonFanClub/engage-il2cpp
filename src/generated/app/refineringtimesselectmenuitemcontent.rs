
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringtimesselectmenuitemcontent/RefineRingTimesSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingTimesSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RefineRingTimesSelectMenuItemContent {
    #[rename(name = "m_TimesText")]
    pub m_times_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BondValueText")]
    pub m_bond_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BondIconImage")]
    pub m_bond_icon_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-refineringtimesselectmenuitemcontent")]
#[::unity2::methods]
impl RefineRingTimesSelectMenuItemContent {
    #[method(name = "get_m_ColorOfNotEnoughValue", args = 0)]
    pub fn get_m_color_of_not_enough_value(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_m_ColorOfNotEnoughValue", args = 1)]
    pub fn set_m_color_of_not_enough_value(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();
}

#[cfg(feature = "app-refineringtimesselectmenuitemcontent")]
impl RefineRingTimesSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingTimesSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingTimesSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
