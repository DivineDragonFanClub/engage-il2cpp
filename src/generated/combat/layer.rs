
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/layer/Layer_Shift.md")))]
#[::unity2::class(namespace = "Combat", name = "Layer.Shift")]
#[parent(crate::system::object::Object)]
pub struct Layer_Shift {
    #[static_field]
    #[rename(name = "contact")]
    pub contact: i32,
}

#[cfg(feature = "combat-layer")]
#[::unity2::methods]
impl Layer_Shift {
    #[method(name = "get_Renderer", args = 0)]
    pub fn get_renderer() -> i32;

    #[method(name = "set_Renderer", args = 1)]
    pub fn set_renderer(value: i32) -> ();

    #[method(name = "get_Character", args = 0)]
    pub fn get_character() -> i32;

    #[method(name = "set_Character", args = 1)]
    pub fn set_character(value: i32) -> ();

    #[method(name = "get_Invisible", args = 0)]
    pub fn get_invisible() -> i32;

    #[method(name = "set_Invisible", args = 1)]
    pub fn set_invisible(value: i32) -> ();

    #[method(name = "get_Ground", args = 0)]
    pub fn get_ground() -> i32;

    #[method(name = "set_Ground", args = 1)]
    pub fn set_ground(value: i32) -> ();

    #[method(name = "get_Shadow", args = 0)]
    pub fn get_shadow() -> i32;

    #[method(name = "set_Shadow", args = 1)]
    pub fn set_shadow(value: i32) -> ();

    #[method(name = "get_Border", args = 0)]
    pub fn get_border() -> i32;

    #[method(name = "set_Border", args = 1)]
    pub fn set_border(value: i32) -> ();

    #[method(name = "get_RedBorder", args = 0)]
    pub fn get_red_border() -> i32;

    #[method(name = "set_RedBorder", args = 1)]
    pub fn set_red_border(value: i32) -> ();

    #[method(name = "get_ArrowsBeforeHit", args = 0)]
    pub fn get_arrows_before_hit() -> i32;

    #[method(name = "get_ArrowsAfterHit", args = 0)]
    pub fn get_arrows_after_hit() -> i32;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/layer/Layer_Mask.md")))]
#[::unity2::class(namespace = "Combat", name = "Layer.Mask")]
#[parent(crate::system::object::Object)]
pub struct Layer_Mask {}

#[cfg(feature = "combat-layer")]
#[::unity2::methods]
impl Layer_Mask {
    #[method(name = "get_Ground", args = 0)]
    pub fn get_ground() -> i32;

    #[method(name = "set_Ground", args = 1)]
    pub fn set_ground(value: i32) -> ();

    #[method(name = "get_Objects", args = 0)]
    pub fn get_objects() -> i32;

    #[method(name = "set_Objects", args = 1)]
    pub fn set_objects(value: i32) -> ();

    #[method(name = "get_Border", args = 0)]
    pub fn get_border() -> i32;

    #[method(name = "set_Border", args = 1)]
    pub fn set_border(value: i32) -> ();

    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera() -> i32;

    #[method(name = "set_Camera", args = 1)]
    pub fn set_camera(value: i32) -> ();

    #[method(name = "get_Hideable", args = 0)]
    pub fn get_hideable() -> i32;

    #[method(name = "set_Hideable", args = 1)]
    pub fn set_hideable(value: i32) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/layer/Layer.md")))]
#[::unity2::class(namespace = "Combat", name = "Layer")]
#[parent(crate::system::object::Object)]
pub struct Layer {}

#[cfg(feature = "combat-layer")]
#[::unity2::methods]
impl Layer {
    #[method(name = "CollidesTo", args = 2)]
    pub fn collides_to(c: crate::unity_engine::collider::Collider, mask: i32) -> bool;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();
}
