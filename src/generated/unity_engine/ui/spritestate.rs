
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/spritestate/SpriteState.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SpriteState {
    pub m_highlighted_sprite: crate::unity_engine::sprite::Sprite,
    pub m_pressed_sprite: crate::unity_engine::sprite::Sprite,
    pub m_selected_sprite: crate::unity_engine::sprite::Sprite,
    pub m_disabled_sprite: crate::unity_engine::sprite::Sprite,
}

impl ::unity2::ClassIdentity for SpriteState {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "SpriteState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SpriteState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-ui-spritestate")]
#[::unity2::methods(value)]
impl SpriteState {
    #[method(name = "get_highlightedSprite", args = 0)]
    pub fn get_highlighted_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_highlightedSprite", args = 1)]
    pub fn set_highlighted_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "get_pressedSprite", args = 0)]
    pub fn get_pressed_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_pressedSprite", args = 1)]
    pub fn set_pressed_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "get_selectedSprite", args = 0)]
    pub fn get_selected_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_selectedSprite", args = 1)]
    pub fn set_selected_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "get_disabledSprite", args = 0)]
    pub fn get_disabled_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_disabledSprite", args = 1)]
    pub fn set_disabled_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::ui::spritestate::SpriteState) -> bool;
}
