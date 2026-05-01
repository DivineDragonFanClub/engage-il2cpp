
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualmenuitemcontent/ProfileCardVisualMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct ProfileCardVisualMenuItemContent {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FixedCursorObject")]
    pub m_fixed_cursor_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FixedCursorImage")]
    pub m_fixed_cursor_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FixedCursorFrameImage")]
    pub m_fixed_cursor_frame_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NewIconObject")]
    pub m_new_icon_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-profilecardvisualmenuitemcontent")]
#[::unity2::methods]
impl ProfileCardVisualMenuItemContent {
    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "SetSprite", args = 1)]
    pub fn set_sprite(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetFixedCursorActive", args = 1)]
    pub fn set_fixed_cursor_active(self, active: bool) -> ();

    #[method(name = "SetNewIconActive", args = 1)]
    pub fn set_new_icon_active(self, active: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardvisualmenuitemcontent")]
impl ProfileCardVisualMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualMenuItemContentMethods>::ctor(this);
        this
    }
}
