
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicmenuitemcontent/BasicMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "BasicMenuItemContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BasicMenuItemContent {
    #[rename(name = "m_menuItem")]
    pub m_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    #[rename(name = "m_textBaseColor")]
    pub m_text_base_color: crate::unity_engine::color::Color,
    #[rename(name = "m_textBlendColor")]
    pub m_text_blend_color: crate::unity_engine::color::Color,
    #[rename(name = "m_frmContent")]
    pub m_frm_content: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-basicmenuitemcontent")]
#[::unity2::methods]
impl BasicMenuItemContent {
    #[method(name = "GetMenuItem", args = 0)]
    pub fn get_menu_item(self) -> crate::app::basicmenuitem::BasicMenuItem;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetText", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetTextSize", args = 1)]
    pub fn set_text_size(self, font_size: i32) -> ();

    #[method(name = "SetTextBaseColor", args = 1)]
    pub fn set_text_base_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetTextBlendColor", args = 1)]
    pub fn set_text_blend_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetFrmContent", args = 1)]
    pub fn set_frm_content(self, frm_content: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_childObject", args = 0)]
    pub fn get_child_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetTextComponent", args = 0)]
    pub fn get_text_component(self) -> crate::unity_engine::ui::text::Text;

    #[method(name = "GetTextMeshProComponent", args = 0)]
    pub fn get_text_mesh_pro_component(self) -> crate::tm_pro::textmeshprougui::TextMeshProUGUI;

    #[method(name = "GetRectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "SetMenuItem", args = 1)]
    pub fn set_menu_item(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildFont", args = 0)]
    pub fn build_font(self) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "BuildTextColor", args = 0)]
    pub fn build_text_color(self) -> ();

    #[method(name = "Unbind", args = 0)]
    pub fn unbind(self) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "ForceRebuildLayout", args = 0)]
    pub fn force_rebuild_layout(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-basicmenuitemcontent")]
impl BasicMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicMenuItemContentMethods>::ctor(this);
        this
    }
}
