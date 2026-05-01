
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/screen/Screen.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Screen")]
#[parent(crate::system::object::Object)]
pub struct Screen {}

#[cfg(feature = "unity_engine-screen")]
#[::unity2::methods]
impl Screen {
    #[method(name = "get_width", args = 0)]
    pub fn get_width() -> i32;

    #[method(name = "get_height", args = 0)]
    pub fn get_height() -> i32;

    #[method(name = "get_dpi", args = 0)]
    pub fn get_dpi() -> f32;

    #[method(name = "get_fullScreenMode", args = 0)]
    pub fn get_full_screen_mode() -> crate::unity_engine::fullscreenmode::FullScreenMode;
}
