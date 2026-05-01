
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenuitemcontent/ShopSellMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ShopSellMenuItemContent {
    #[rename(name = "m_FixedCursorObject")]
    pub m_fixed_cursor_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FixedCursorFrameObject")]
    pub m_fixed_cursor_frame_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KindFrameObject")]
    pub m_kind_frame_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KindIconObject")]
    pub m_kind_icon_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NameObject")]
    pub m_name_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CountObject")]
    pub m_count_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PriceValueObject")]
    pub m_price_value_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PriceGObject")]
    pub m_price_g_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitIconObject")]
    pub m_unit_icon_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_textBaseColor2")]
    pub m_text_base_color2: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-shopsellmenuitemcontent")]
#[::unity2::methods]
impl ShopSellMenuItemContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "SetTextBaseColor", args = 2)]
    pub fn set_text_base_color(
        self,
        color: crate::unity_engine::color::Color,
        color2: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "ShowFixedCursor", args = 0)]
    pub fn show_fixed_cursor(self) -> ();

    #[method(name = "HideFixedCursor", args = 0)]
    pub fn hide_fixed_cursor(self) -> ();
}

#[cfg(feature = "app-shopsellmenuitemcontent")]
impl ShopSellMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuItemContentMethods>::ctor(this);
        this
    }
}
