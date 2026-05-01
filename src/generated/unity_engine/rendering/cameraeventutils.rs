
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/cameraeventutils/CameraEventUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "CameraEventUtils")]
#[parent(crate::system::object::Object)]
pub struct CameraEventUtils {}

#[cfg(feature = "unity_engine-rendering-cameraeventutils")]
#[::unity2::methods]
impl CameraEventUtils {
    #[method(name = "IsValid", args = 1)]
    pub fn is_valid(value: crate::unity_engine::rendering::cameraevent::CameraEvent) -> bool;
}
