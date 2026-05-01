
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/hudpopup/HUDPopup.md")))]
#[::unity2::class(namespace = "Combat", name = "HUDPopup")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HUDPopup {
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_WorldPos")]
    pub m_world_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Offset2D")]
    pub m_offset2_d: crate::unity_engine::vector2::Vector2,
}

#[cfg(feature = "combat-hudpopup")]
#[::unity2::methods]
impl HUDPopup {
    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetWorldPosition", args = 1)]
    pub fn set_world_position(
        self,
        world_pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "SetWorldPositionAndDisplayNumber", args = 2)]
    pub fn set_world_position_and_display_number(
        self,
        world_pos: crate::unity_engine::vector3::Vector3,
        value: i32,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "Set2DOffset", args = 2)]
    pub fn set2_d_offset(self, offset_x: f32, offset_y: f32) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, text: ::unity2::Il2CppString) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "SetText", args = 1)]
    pub fn set_text_2(self, value: i32) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "GetText", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "SetTextColor", args = 1)]
    pub fn set_text_color(
        self,
        color: crate::unity_engine::color::Color,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = "SetImage", args = 2)]
    pub fn set_image(
        self,
        name: ::unity2::Il2CppString,
        sprite: crate::unity_engine::sprite::Sprite,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-hudpopup")]
impl HUDPopup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HUDPopup),
                ::core::stringify!(new),
            )
        });
        <Self as IHUDPopupMethods>::ctor(this);
        this
    }
}
