
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/imask_interface/IMask_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "IMask")]
pub struct IMask_Interface {}

#[cfg(feature = "unity_engine-ui-imask_interface")]
#[::unity2::methods]
impl IMask_Interface {
    #[method(name = "Enabled", args = 0)]
    pub fn enabled(self) -> bool;

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;
}
