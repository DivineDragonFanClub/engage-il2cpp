
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/setpropertyutility_2/SetPropertyUtility_2.md")))]
#[::unity2::class(namespace = "TMPro", name = "SetPropertyUtility")]
#[parent(crate::system::object::Object)]
pub struct SetPropertyUtility_2 {}

#[cfg(feature = "tm_pro-setpropertyutility_2")]
#[::unity2::methods]
impl SetPropertyUtility_2 {
    #[method(name = "SetColor", args = 2)]
    pub fn set_color(
        current_value: crate::unity_engine::color::Color,
        new_value: crate::unity_engine::color::Color,
    ) -> bool;
}
