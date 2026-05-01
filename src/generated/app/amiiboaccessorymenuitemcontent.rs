
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiiboaccessorymenuitemcontent/AmiiboAccessoryMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboAccessoryMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct AmiiboAccessoryMenuItemContent {
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_TicketObj")]
    pub m_ticket_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TicketIcon")]
    pub m_ticket_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_TicketValue")]
    pub m_ticket_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NewObj")]
    pub m_new_obj: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "c_Cost")]
    pub c_cost: i32,
    #[rename(name = "m_IsCanBy")]
    pub m_is_can_by: bool,
}

#[cfg(feature = "app-amiiboaccessorymenuitemcontent")]
#[::unity2::methods]
impl AmiiboAccessoryMenuItemContent {
    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-amiiboaccessorymenuitemcontent")]
impl AmiiboAccessoryMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboAccessoryMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboAccessoryMenuItemContentMethods>::ctor(this);
        this
    }
}
