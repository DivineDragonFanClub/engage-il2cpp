
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringselectmenuitemcontent/RingSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RingSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RingSelectMenuItemContent {
    #[rename(name = "m_name")]
    pub m_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_emptyText")]
    pub m_empty_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_face")]
    pub m_face: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_symbol")]
    pub m_symbol: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_selectBg")]
    pub m_select_bg: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_frame")]
    pub m_frame: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_check")]
    pub m_check: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_unitIcon")]
    pub m_unit_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsEquip")]
    pub m_is_equip: bool,
    #[rename(name = "m_IsEmpty")]
    pub m_is_empty: bool,
    #[rename(name = "m_numRoot")]
    pub m_num_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_num")]
    pub m_num: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_x")]
    pub m_x: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-ringselectmenuitemcontent")]
#[::unity2::methods]
impl RingSelectMenuItemContent {
    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "SetFaceImage", args = 1)]
    pub fn set_face_image(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "SetSymbolImage", args = 1)]
    pub fn set_symbol_image(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "SetOwner", args = 1)]
    pub fn set_owner(self, owner: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringselectmenuitemcontent")]
impl RingSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRingSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
