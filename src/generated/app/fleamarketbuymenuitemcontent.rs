
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
use crate::app::itemshopbuymenuitemcontent::IItemShopBuyMenuItemContent;
use crate::app::itemshopbuymenuitemcontent::ItemShopBuyMenuItemContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fleamarketbuymenuitemcontent/FleaMarketBuyMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "FleaMarketBuyMenuItemContent")]
#[parent(crate::app::itemshopbuymenuitemcontent::ItemShopBuyMenuItemContent)]
pub struct FleaMarketBuyMenuItemContent {
    #[rename(name = "m_KindIconImage")]
    pub m_kind_icon_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-fleamarketbuymenuitemcontent")]
#[::unity2::methods]
impl FleaMarketBuyMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fleamarketbuymenuitemcontent")]
impl FleaMarketBuyMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FleaMarketBuyMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFleaMarketBuyMenuItemContentMethods>::ctor(this);
        this
    }
}
