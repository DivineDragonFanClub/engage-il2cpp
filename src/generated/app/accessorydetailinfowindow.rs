
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorydetailinfowindow/AccessoryDetailInfoWindow_BodyParts.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryDetailInfoWindow.BodyParts")]
#[parent(crate::system::object::Object)]
pub struct AccessoryDetailInfoWindow_BodyParts {
    #[rename(name = "m_Object")]
    pub m_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Text")]
    pub m_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-accessorydetailinfowindow")]
#[::unity2::methods]
impl AccessoryDetailInfoWindow_BodyParts {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessorydetailinfowindow")]
impl AccessoryDetailInfoWindow_BodyParts {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryDetailInfoWindow_BodyParts),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryDetailInfoWindow_BodyPartsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessorydetailinfowindow/AccessoryDetailInfoWindow.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryDetailInfoWindow")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct AccessoryDetailInfoWindow {
    #[rename(name = "m_AccessoryName")]
    pub m_accessory_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Message")]
    pub m_message: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BodyParts")]
    pub m_body_parts:
        ::unity2::Array<crate::app::accessorydetailinfowindow::AccessoryDetailInfoWindow_BodyParts>,
}

#[cfg(feature = "app-accessorydetailinfowindow")]
#[::unity2::methods]
impl AccessoryDetailInfoWindow {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "SetData", args = 2)]
    pub fn set_data(
        self,
        accessory_data: crate::app::accessorydata::AccessoryData,
        female: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
    ) -> ();
}

#[cfg(feature = "app-accessorydetailinfowindow")]
impl AccessoryDetailInfoWindow {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryDetailInfoWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryDetailInfoWindowMethods>::ctor(this);
        this
    }
}
