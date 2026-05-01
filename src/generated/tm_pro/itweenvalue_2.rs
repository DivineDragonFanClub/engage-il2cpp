
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/itweenvalue_2/ITweenValue_2.md")))]
#[::unity2::class(namespace = "TMPro", name = "ITweenValue")]
pub struct ITweenValue_2 {}

#[cfg(feature = "tm_pro-itweenvalue_2")]
#[::unity2::methods]
impl ITweenValue_2 {
    #[method(name = "TweenValue", args = 1)]
    pub fn tween_value(self, float_percentage: f32) -> ();

    #[method(name = "get_ignoreTimeScale", args = 0)]
    pub fn get_ignore_time_scale(self) -> bool;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f32;

    #[method(name = "ValidTarget", args = 0)]
    pub fn valid_target(self) -> bool;
}
