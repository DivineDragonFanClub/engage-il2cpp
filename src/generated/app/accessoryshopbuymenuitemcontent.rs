
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenuitemcontent/AccessoryShopBuyMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct AccessoryShopBuyMenuItemContent {
    #[rename(name = "m_KindIconObject")]
    pub m_kind_icon_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KindIconImage")]
    pub m_kind_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NameObject")]
    pub m_name_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NameText")]
    pub m_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_IronObject")]
    pub m_iron_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IronIconImage")]
    pub m_iron_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_IronValueText")]
    pub m_iron_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SteelObject")]
    pub m_steel_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SteelIconImage")]
    pub m_steel_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SteelValueText")]
    pub m_steel_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SilverObject")]
    pub m_silver_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SilverIconImage")]
    pub m_silver_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SilverValueText")]
    pub m_silver_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PriceObject")]
    pub m_price_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PriceIconImage")]
    pub m_price_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PriceValueText")]
    pub m_price_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PriceGText")]
    pub m_price_g_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NewIconObject")]
    pub m_new_icon_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-accessoryshopbuymenuitemcontent")]
#[::unity2::methods]
impl AccessoryShopBuyMenuItemContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "SynchronizeNewIconAnimationTime", args = 0)]
    pub fn synchronize_new_icon_animation_time(self) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenuitemcontent")]
impl AccessoryShopBuyMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenuItemContentMethods>::ctor(this);
        this
    }
}
