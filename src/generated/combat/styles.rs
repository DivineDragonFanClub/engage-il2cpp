
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/styles/Styles.md")))]
#[::unity2::class(namespace = "Combat", name = "Styles")]
#[parent(crate::system::object::Object)]
pub struct Styles {}

#[cfg(feature = "combat-styles")]
#[::unity2::methods]
impl Styles {
    #[method(name = "get_Scale", args = 0)]
    pub fn get_scale() -> f32;

    #[method(name = "FitToScreen", args = 1)]
    pub fn fit_to_screen(style: crate::unity_engine::guistyle::GUIStyle) -> ();
}
